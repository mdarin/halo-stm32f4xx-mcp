use clap::Command as ClapCommand;

use super::Command;
use crate::models::{LintResponse, Response, Summary, ToolkitError, SCHEMA_VERSION};

pub struct Lint;

impl Lint {
    pub(crate) fn placeholder(command: &str, flow: bool) -> LintResponse {
        LintResponse {
            schema_version: SCHEMA_VERSION,
            command: command.into(),
            violations: vec![],
            flows: if flow { Some(vec![]) } else { None },
            summary: Summary::default(),
        }
    }
}

impl Command for Lint {
    fn name(&self) -> &'static str {
        "lint"
    }

    fn about(&self) -> &'static str {
        "Run F407 lint rules"
    }

    fn clap_command(&self) -> ClapCommand {
        ClapCommand::new(self.name())
            .about(self.about())
            .arg(super::file_flag())
            .arg(super::flow_flag())
    }

    fn run(&self, matches: &clap::ArgMatches) -> Result<Response, ToolkitError> {
        let code = super::code_from(matches, self.name())?;
        let _ = code;
        let flow = matches.get_flag("flow");
        Ok(Response::Lint(Self::placeholder(self.name(), flow)))
    }
}
