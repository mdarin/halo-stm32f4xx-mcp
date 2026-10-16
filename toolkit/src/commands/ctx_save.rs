use clap::{Arg, Command as ClapCommand};

use super::Command;
use crate::config::Config;
use crate::context::{self, ProjectContext};
use crate::models::{CtxSaveResponse, Response, SCHEMA_VERSION};

pub struct CtxSave;

/// Вход ctx-save (stdin JSON, CONTRACT.md §10).
#[derive(serde::Deserialize)]
struct CtxSaveInput {
    project_path: String,
    entries: Vec<CtxEntryInput>,
}

#[derive(serde::Deserialize)]
struct CtxEntryInput {
    key: String,
    #[serde(default)]
    doc_type: Option<String>,
    text: String,
    #[serde(default)]
    peripherals: Vec<String>,
    #[serde(default)]
    interrupts: Vec<String>,
    #[serde(default)]
    user_preferences: std::collections::HashMap<String, String>,
    #[serde(default)]
    modification: Option<context::Modification>,
}

impl Command for CtxSave {
    fn name(&self) -> &'static str {
        "ctx-save"
    }

    fn about(&self) -> &'static str {
        "Save agent context entries for a project (Qdrant project_context collection)"
    }

    fn clap_command(&self) -> ClapCommand {
        ClapCommand::new(self.name()).about(self.about()).arg(
            Arg::new("stdin")
                .long("stdin")
                .action(clap::ArgAction::SetTrue)
                .help("Read input JSON from stdin (default)"),
        )
    }

    fn run(&self, _matches: &clap::ArgMatches) -> Result<Response, crate::models::ToolkitError> {
        let raw = super::read_stdin(self.name())?;
        let input: CtxSaveInput = serde_json::from_str(&raw).map_err(|e| {
            crate::models::ToolkitError::new(
                crate::models::ErrorCode::Parse,
                format!("invalid ctx-save input: {e}"),
                self.name(),
            )
        })?;

        let entries: Vec<ProjectContext> = input
            .entries
            .into_iter()
            .map(|e| ProjectContext {
                project_path: input.project_path.clone(),
                key: e.key,
                doc_type: e
                    .doc_type
                    .unwrap_or_else(|| context::DOC_TYPE_CONTEXT.into()),
                text: e.text,
                peripherals: e.peripherals,
                interrupts: e.interrupts,
                user_preferences: e.user_preferences,
                modification: e.modification,
                schema_version: 1,
            })
            .collect();

        let cfg = Config::from_env();
        let collection = context::collection_from_env();
        let saved = context::save(&cfg, &collection, entries).map_err(|e| {
            crate::models::ToolkitError::new(
                crate::models::ErrorCode::RagUnavailable,
                e,
                self.name(),
            )
        })?;

        Ok(Response::CtxSave(CtxSaveResponse {
            schema_version: SCHEMA_VERSION,
            command: self.name().into(),
            saved,
            collection,
        }))
    }
}
