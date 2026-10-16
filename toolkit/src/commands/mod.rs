use clap::{Arg, ArgAction, Command as ClapCommand};
use std::io::Read;

use crate::models::{Response, ToolkitError};

pub mod analyze;
pub mod flow_analysis;
pub mod generate_docs;
pub mod lint;
pub mod rag_search;

#[allow(dead_code)]
pub const CONTRACT_VERSION: &str = "1.0";

pub trait Command: Sync {
    fn name(&self) -> &'static str;
    fn about(&self) -> &'static str;
    fn clap_command(&self) -> ClapCommand;
    fn run(&self, matches: &clap::ArgMatches) -> Result<Response, ToolkitError>;
}

pub fn registry() -> Vec<Box<dyn Command>> {
    vec![
        Box::new(analyze::Analyze),
        Box::new(lint::Lint),
        Box::new(flow_analysis::FlowAnalysis),
        Box::new(generate_docs::GenerateDocs),
        Box::new(rag_search::RagSearch),
    ]
}

pub fn build_root(commands: &[Box<dyn Command>]) -> ClapCommand {
    let mut root = ClapCommand::new("toolkit")
        .version(concat!(env!("CARGO_PKG_VERSION"), " (contract 1.0)"))
        .about("STM32F407 development toolkit")
        .subcommand_required(true)
        .arg_required_else_help(true);
    for cmd in commands {
        root = root.subcommand(cmd.clap_command());
    }
    root
}

pub fn dispatch(
    commands: &[Box<dyn Command>],
    matches: &clap::ArgMatches,
) -> Result<Response, ToolkitError> {
    let (name, sub) = matches.subcommand().ok_or_else(|| {
        ToolkitError::new(
            crate::models::ErrorCode::Internal,
            "no subcommand given",
            "",
        )
    })?;
    commands
        .iter()
        .find(|c| c.name() == name)
        .ok_or_else(|| {
            ToolkitError::new(
                crate::models::ErrorCode::Internal,
                format!("unknown subcommand {name}"),
                name,
            )
        })?
        .run(sub)
}

pub fn read_code(file: Option<&str>, command: &str) -> Result<String, ToolkitError> {
    match file {
        Some(path) => std::fs::read_to_string(path).map_err(|e| {
            ToolkitError::new(
                crate::models::ErrorCode::InputRead,
                format!("failed to read file {path}: {e}"),
                command,
            )
        }),
        None => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf).map_err(|e| {
                ToolkitError::new(
                    crate::models::ErrorCode::InputRead,
                    format!("failed to read stdin: {e}"),
                    command,
                )
            })?;
            Ok(buf)
        }
    }
}

pub fn code_from(matches: &clap::ArgMatches, command: &str) -> Result<String, ToolkitError> {
    read_code(
        matches.get_one::<String>("file").map(|s| s.as_str()),
        command,
    )
}

fn file_flag() -> Arg {
    Arg::new("file")
        .long("file")
        .value_name("PATH")
        .help("Read code from file instead of stdin")
}

fn flow_flag() -> Arg {
    Arg::new("flow")
        .long("flow")
        .action(ArgAction::SetTrue)
        .help("Track variable usage in DMA paths")
}

fn style_flag() -> Arg {
    Arg::new("style")
        .long("style")
        .value_name("STYLE")
        .default_value("doxygen")
        .help("Comment style")
}

fn rule_context_flag() -> Arg {
    Arg::new("rule_context")
        .long("rule-context")
        .value_name("CONTEXT")
        .default_value("")
        .help("Rule context to embed alongside the code")
}

fn limit_flag() -> Arg {
    Arg::new("limit")
        .long("limit")
        .value_name("N")
        .default_value("5")
        .value_parser(clap::value_parser!(usize))
        .help("Max RAG references")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_version_is_major_minor() {
        let parts: Vec<&str> = CONTRACT_VERSION.split('.').collect();
        assert_eq!(parts.len(), 2);
        assert!(parts.iter().all(|p| p.parse::<u32>().is_ok()));
    }

    #[test]
    fn registry_names_are_unique() {
        let cmds = registry();
        let mut names: Vec<&str> = cmds.iter().map(|c| c.name()).collect();
        let total = names.len();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), total);
    }
}
