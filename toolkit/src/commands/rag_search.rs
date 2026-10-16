use clap::Command as ClapCommand;

use super::{code_from, Command};
use crate::config::Config;
use crate::models::{Response, ToolkitError};
use crate::rag;

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
        let rule_context = matches
            .get_one::<String>("rule_context")
            .cloned()
            .unwrap_or_default();
        let limit = matches.get_one::<usize>("limit").copied().unwrap_or(5);
        let cfg = Config::from_env();
        let response = rag::rt().block_on(rag::run(&code, &rule_context, limit, &cfg));
        Ok(Response::RagSearch(response))
    }
}
