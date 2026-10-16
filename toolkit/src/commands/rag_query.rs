use clap::{Arg, Command as ClapCommand};

use super::Command;
use crate::config::Config;
use crate::models::{RagQueryResponse, Response, SCHEMA_VERSION};
use crate::rag;

pub struct RagQuery;

impl Command for RagQuery {
    fn name(&self) -> &'static str {
        "rag-query"
    }

    fn about(&self) -> &'static str {
        "Semantic search over indexed hardware documentation (Qdrant/Ollama)"
    }

    fn clap_command(&self) -> ClapCommand {
        ClapCommand::new(self.name())
            .about(self.about())
            .arg(
                Arg::new("query")
                    .long("query")
                    .short('q')
                    .value_name("TEXT")
                    .required(true)
                    .help("Search query text"),
            )
            .arg(
                Arg::new("peripheral")
                    .long("peripheral")
                    .value_name("NAME")
                    .default_value("")
                    .help("Optional filter by peripheral (GPIO, TIM, DMA, ...)"),
            )
            .arg(super::limit_flag())
    }

    fn run(&self, matches: &clap::ArgMatches) -> Result<Response, crate::models::ToolkitError> {
        let query = matches
            .get_one::<String>("query")
            .map(|s| s.as_str())
            .unwrap_or_default()
            .to_string();
        let peripheral = matches
            .get_one::<String>("peripheral")
            .cloned()
            .unwrap_or_default();
        let limit = matches.get_one::<usize>("limit").copied().unwrap_or(5);
        let cfg = Config::from_env();

        let results = rag::query(&query, &peripheral, limit, &cfg).map_err(|e| {
            crate::models::ToolkitError::new(
                crate::models::ErrorCode::RagUnavailable,
                e,
                self.name(),
            )
        })?;

        Ok(Response::RagQuery(RagQueryResponse {
            schema_version: SCHEMA_VERSION,
            command: self.name().into(),
            query,
            peripheral,
            results,
        }))
    }
}
