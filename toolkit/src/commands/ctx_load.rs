use clap::{Arg, Command as ClapCommand};

use super::Command;
use crate::config::Config;
use crate::context;
use crate::models::{CtxLoadResponse, Response, SCHEMA_VERSION};

pub struct CtxLoad;

impl Command for CtxLoad {
    fn name(&self) -> &'static str {
        "ctx-load"
    }

    fn about(&self) -> &'static str {
        "Load stored context entries for a project (optionally one key)"
    }

    fn clap_command(&self) -> ClapCommand {
        ClapCommand::new(self.name())
            .about(self.about())
            .arg(
                Arg::new("project")
                    .long("project")
                    .value_name("PATH")
                    .required(true)
                    .help("Absolute project path"),
            )
            .arg(
                Arg::new("key")
                    .long("key")
                    .value_name("KEY")
                    .help("Load a single ctx_key instead of the whole project"),
            )
    }

    fn run(&self, matches: &clap::ArgMatches) -> Result<Response, crate::models::ToolkitError> {
        let project = matches
            .get_one::<String>("project")
            .map(|s| s.as_str())
            .unwrap_or_default()
            .to_string();
        let key = matches.get_one::<String>("key").map(|s| s.as_str());

        let cfg = Config::from_env();
        let collection = context::collection_from_env();
        let entries = context::load(&cfg, &collection, &project, key).map_err(|e| {
            crate::models::ToolkitError::new(
                crate::models::ErrorCode::RagUnavailable,
                e,
                self.name(),
            )
        })?;

        Ok(Response::CtxLoad(CtxLoadResponse {
            schema_version: SCHEMA_VERSION,
            command: self.name().into(),
            project_path: project,
            count: entries.len(),
            entries,
        }))
    }
}
