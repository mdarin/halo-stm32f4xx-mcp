use clap::{Arg, Command as ClapCommand};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use super::Command;
use crate::analyzer::Analyzer;
use crate::config::Config;
use crate::context;
use crate::models::{CtxBlock, CtxExtractResponse, Response, SCHEMA_VERSION};

pub struct CtxExtract;

/// Каталоги, которые не сканируем (build-артефакты, HAL, VCS).
const SKIP_DIRS: &[&str] = &[
    ".git",
    ".pio",
    ".pioenvs",
    ".vscode",
    "build",
    "Build",
    "Debug",
    "Release",
    "Drivers",
    "Middlewares",
    "node_modules",
];

fn find_ioc(project: &Path) -> Option<PathBuf> {
    let mut candidates: Vec<PathBuf> = std::fs::read_dir(project)
        .ok()?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|x| x == "ioc").unwrap_or(false))
        .collect();
    candidates.sort();
    candidates.into_iter().next()
}

/// Периферия из .ioc: значения ключей `Mcu.IP<N>`, MCU — `Mcu.Name`.
fn parse_ioc(path: &Path) -> (Vec<String>, Option<String>) {
    let mut peripherals = BTreeSet::new();
    let mut mcu = None;
    if let Ok(text) = std::fs::read_to_string(path) {
        for line in text.lines() {
            if let Some(rest) = line.strip_prefix("Mcu.Name=") {
                mcu = Some(rest.trim().to_string());
            } else if line.starts_with("Mcu.IP") && !line.starts_with("Mcu.IPNb=") {
                let rest = &line["Mcu.IP".len()..];
                if let Some(value) = rest.split('=').nth(1) {
                    let v = value.trim();
                    if !v.is_empty() {
                        peripherals.insert(v.to_string());
                    }
                }
            }
        }
    }
    (peripherals.into_iter().collect(), mcu)
}

/// Рекурсивный обход исходников проекта (только .c — IRQ-обработчики
/// определяются в .c; заголовки пользовательского кода читает клиент).
fn collect_sources(project: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(project) else {
        return;
    };
    let mut dirs = Vec::new();
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if !SKIP_DIRS.contains(&name) && !name.starts_with('.') {
                dirs.push(path);
            }
        } else if path.extension().map(|x| x == "c").unwrap_or(false) {
            out.push(path);
        }
    }
    for dir in dirs {
        collect_sources(&dir, out);
    }
}

/// `/* USER CODE BEGIN <name> */` в сыром тексте.
fn user_code_blocks(source: &str) -> Vec<String> {
    let prefix = "/* USER CODE BEGIN ";
    let mut names = Vec::new();
    for line in source.lines() {
        if let Some(idx) = line.find(prefix) {
            let rest = &line[idx + prefix.len()..];
            if let Some(end) = rest.find(" */") {
                let name = rest[..end].trim();
                if !name.is_empty() {
                    names.push(name.to_string());
                }
            }
        }
    }
    names
}

impl Command for CtxExtract {
    fn name(&self) -> &'static str {
        "ctx-extract"
    }

    fn about(&self) -> &'static str {
        "Extract peripherals (.ioc), IRQ handlers (AST) and USER CODE blocks from a project"
    }

    fn clap_command(&self) -> ClapCommand {
        ClapCommand::new(self.name()).about(self.about()).arg(
            Arg::new("project")
                .long("project")
                .value_name("PATH")
                .required(true)
                .help("Project root (looks for *.ioc, scans sources recursively)"),
        )
    }

    fn run(&self, matches: &clap::ArgMatches) -> Result<Response, crate::models::ToolkitError> {
        let project = matches
            .get_one::<String>("project")
            .map(|s| s.as_str())
            .unwrap_or_default();
        let root = Path::new(project);
        if !root.is_dir() {
            return Err(crate::models::ToolkitError::new(
                crate::models::ErrorCode::InputRead,
                format!("project dir not found: {project}"),
                self.name(),
            ));
        }

        let (peripherals, mcu) = find_ioc(root)
            .map(|ioc| parse_ioc(&ioc))
            .unwrap_or((Vec::new(), None));

        let mut sources = Vec::new();
        collect_sources(root, &mut sources);
        sources.sort();

        let mut analyzer = Analyzer::new();
        let mut interrupts = BTreeSet::new();
        let mut files = Vec::new();
        let mut blocks = Vec::new();
        for src in &sources {
            let Ok(text) = std::fs::read_to_string(src) else {
                continue;
            };
            for f in analyzer.analyze(&text).functions {
                if f.name.ends_with("_IRQHandler") {
                    interrupts.insert(f.name);
                }
            }
            let block_names = user_code_blocks(&text);
            if !block_names.is_empty() {
                let rel = src
                    .strip_prefix(root)
                    .unwrap_or(src)
                    .to_string_lossy()
                    .to_string();
                files.push(rel.clone());
                for name in block_names {
                    blocks.push(CtxBlock {
                        file: rel.clone(),
                        name,
                    });
                }
            }
        }

        // Diff против сохранённого контекста — только если Qdrant живой.
        let cfg = Config::from_env();
        let collection = context::collection_from_env();
        let (new_peripherals, new_interrupts) =
            match context::load(&cfg, &collection, project, None) {
                Ok(entries) => {
                    let saved_p: BTreeSet<String> = entries
                        .iter()
                        .flat_map(|e| e.peripherals.iter().cloned())
                        .collect();
                    let saved_i: BTreeSet<String> = entries
                        .iter()
                        .flat_map(|e| e.interrupts.iter().cloned())
                        .collect();
                    let np: Vec<String> = peripherals
                        .iter()
                        .filter(|p| !saved_p.contains(*p))
                        .cloned()
                        .collect();
                    let ni: Vec<String> = interrupts
                        .iter()
                        .filter(|i| !saved_i.contains(*i))
                        .cloned()
                        .collect();
                    (Some(np), Some(ni))
                }
                Err(_) => (None, None),
            };

        Ok(Response::CtxExtract(CtxExtractResponse {
            schema_version: SCHEMA_VERSION,
            command: self.name().into(),
            project_path: project.into(),
            mcu,
            peripherals,
            interrupts: interrupts.into_iter().collect(),
            files,
            blocks,
            new_peripherals,
            new_interrupts,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ioc_reads_mcu_ip_lines() {
        let dir = std::env::temp_dir().join(format!("ctxextract-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let ioc = dir.join("Test.ioc");
        std::fs::write(
            &ioc,
            "Mcu.Name=STM32F407Z(E-G)Tx\nMcu.IP0=ADC1\nMcu.IP1=DMA\nMcu.IP2=TIM3\nMcu.IPNb=3\nNVIC.X=1\n",
        )
        .unwrap();
        let (peripherals, mcu) = parse_ioc(&ioc);
        assert_eq!(mcu.as_deref(), Some("STM32F407Z(E-G)Tx"));
        assert_eq!(peripherals, vec!["ADC1", "DMA", "TIM3"]);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn user_code_blocks_scan() {
        let src = "/* USER CODE BEGIN Includes */\n#include \"x.h\"\n/* USER CODE END Includes */\n/* USER CODE BEGIN 4 */\n/* USER CODE END 4 */\n/* USER CODE BEGIN Private defines */";
        let blocks = user_code_blocks(src);
        assert_eq!(blocks, vec!["Includes", "4", "Private defines"]);
    }

    #[test]
    fn collect_sources_skips_build_dirs() {
        let dir = std::env::temp_dir().join(format!("ctxextract2-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("src")).unwrap();
        std::fs::create_dir_all(dir.join(".pio/build")).unwrap();
        std::fs::create_dir_all(dir.join("Drivers/HAL")).unwrap();
        std::fs::write(dir.join("src/main.c"), "").unwrap();
        std::fs::write(dir.join(".pio/build/x.o"), "").unwrap();
        std::fs::write(dir.join("Drivers/hal.c"), "").unwrap();
        let mut out = Vec::new();
        collect_sources(&dir, &mut out);
        assert_eq!(out, vec![dir.join("src/main.c")]);
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
