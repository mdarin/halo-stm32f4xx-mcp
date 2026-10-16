use clap::{Arg, Command as ClapCommand};

use super::Command;
use crate::config::Config;
use crate::models::{RagIndexResponse, Response, SCHEMA_VERSION};
use crate::rag;

pub struct RagIndex;

impl Command for RagIndex {
    fn name(&self) -> &'static str {
        "rag-index"
    }

    fn about(&self) -> &'static str {
        "Index a kb directory (manifest.json) into Qdrant: chunk, embed, upsert"
    }

    fn clap_command(&self) -> ClapCommand {
        ClapCommand::new(self.name())
            .about(self.about())
            .arg(
                Arg::new("path")
                    .long("path")
                    .value_name("DIR")
                    .required(true)
                    .help("Directory with manifest.json and markdown chunks"),
            )
    }

    fn run(&self, matches: &clap::ArgMatches) -> Result<Response, crate::models::ToolkitError> {
        let path = matches
            .get_one::<String>("path")
            .map(|s| s.as_str())
            .unwrap_or_default();
        let cfg = Config::from_env();

        let stats = rag::index_kb(std::path::Path::new(path), &cfg).map_err(|e| {
            crate::models::ToolkitError::new(
                crate::models::ErrorCode::RagUnavailable,
                e,
                self.name(),
            )
        })?;

        Ok(Response::RagIndex(RagIndexResponse {
            schema_version: SCHEMA_VERSION,
            command: self.name().into(),
            files: stats.files,
            chunks: stats.chunks,
            skipped: stats.skipped,
            errors: stats.errors,
        }))
    }
}
