use clap::Command as ClapCommand;

use super::{code_from, Command};
use crate::models::{AnalyzeResponse, Response, SCHEMA_VERSION, SourceStats, ToolkitError};

pub struct Analyze;

impl Command for Analyze {
    fn name(&self) -> &'static str {
        "analyze"
    }

    fn about(&self) -> &'static str {
        "Parse C source and extract AST-level facts"
    }

    fn clap_command(&self) -> ClapCommand {
        ClapCommand::new(self.name()).about(self.about()).arg(super::file_flag())
    }

    fn run(&self, matches: &clap::ArgMatches) -> Result<Response, ToolkitError> {
        let code = code_from(matches, self.name())?;
        Ok(Response::Analyze(AnalyzeResponse {
            schema_version: SCHEMA_VERSION,
            command: self.name().into(),
            source_stats: SourceStats { bytes: code.len(), lines: code.lines().count().max(if code.is_empty() { 0 } else { 1 }) },
            functions: vec![],
            variables: vec![],
            hal_calls: vec![],
            parse_errors: vec![],
        }))
    }
}
