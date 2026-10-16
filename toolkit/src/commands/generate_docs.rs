use clap::Command as ClapCommand;

use super::{code_from, Command};
use crate::models::{DocsResponse, Response, ToolkitError, SCHEMA_VERSION};

pub struct GenerateDocs;

impl Command for GenerateDocs {
    fn name(&self) -> &'static str {
        "generate-docs"
    }

    fn about(&self) -> &'static str {
        "Generate doc-comment insertions"
    }

    fn clap_command(&self) -> ClapCommand {
        ClapCommand::new(self.name())
            .about(self.about())
            .arg(super::file_flag())
            .arg(super::style_flag())
    }

    fn run(&self, matches: &clap::ArgMatches) -> Result<Response, ToolkitError> {
        let code = code_from(matches, self.name())?;
        let _style = matches.get_one::<String>("style");
        let _ = code;
        Ok(Response::GenerateDocs(DocsResponse {
            schema_version: SCHEMA_VERSION,
            command: self.name().into(),
            insertions: vec![],
            functions_documented: 0,
            functions_skipped: 0,
        }))
    }
}
