use clap::{Arg, Command as ClapCommand};

use super::Command;
use crate::config::Config;
use crate::context;
use crate::models::{CtxSearchResponse, Response, SCHEMA_VERSION};

pub struct CtxSearch;

impl Command for CtxSearch {
    fn name(&self) -> &'static str {
        "ctx-search"
    }

    fn about(&self) -> &'static str {
        "Semantic search over stored agent context for a project"
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
                Arg::new("query")
                    .long("query")
                    .short('q')
                    .value_name("TEXT")
                    .required(true)
                    .help("Search query text"),
            )
            .arg(super::limit_flag())
    }

    fn run(&self, matches: &clap::ArgMatches) -> Result<Response, crate::models::ToolkitError> {
        let project = matches
            .get_one::<String>("project")
            .map(|s| s.as_str())
            .unwrap_or_default()
            .to_string();
        let query = matches
            .get_one::<String>("query")
            .map(|s| s.as_str())
            .unwrap_or_default()
            .to_string();
        let limit = matches.get_one::<usize>("limit").copied().unwrap_or(5);

        let cfg = Config::from_env();
        let collection = context::collection_from_env();
        let results = context::search(&cfg, &collection, &project, &query, limit).map_err(|e| {
            crate::models::ToolkitError::new(
                crate::models::ErrorCode::RagUnavailable,
                e,
                self.name(),
            )
        })?;

        Ok(Response::CtxSearch(CtxSearchResponse {
            schema_version: SCHEMA_VERSION,
            command: self.name().into(),
            project_path: project,
            query,
            results,
        }))
    }
}
