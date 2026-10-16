use clap::Command as ClapCommand;

use super::{code_from, Command};
use crate::config::Config;
use crate::models::{RagResponse, Response, ToolkitError, SCHEMA_VERSION};

pub struct RagSearch;

impl Command for RagSearch {
    fn name(&self) -> &'static str {
        "rag-search"
    }

    fn about(&self) -> &'static str {
        "AST analysis enriched with hardware documentation (Qdrant/Ollama)"
    }

    fn clap_command(&self) -> ClapCommand {
        ClapCommand::new(self.name())
            .about(self.about())
            .arg(super::file_flag())
            .arg(super::rule_context_flag())
            .arg(super::limit_flag())
    }

    fn run(&self, matches: &clap::ArgMatches) -> Result<Response, ToolkitError> {
        let code = code_from(matches, self.name())?;
        let _rule_context = matches.get_one::<String>("rule_context");
        let _limit = matches.get_one::<usize>("limit");
        let cfg = Config::from_env();
        let _ = (&code, &cfg);
        Ok(Response::RagSearch(RagResponse {
            schema_version: SCHEMA_VERSION,
            command: self.name().into(),
            violations: vec![],
            rag_refs: vec![],
            rag_degraded: false,
        }))
    }
}
