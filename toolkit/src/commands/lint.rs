use clap::Command as ClapCommand;

use super::Command;
use crate::lint;
use crate::models::{LintResponse, Response, ToolkitError, SCHEMA_VERSION};

pub struct Lint;

impl Lint {
    pub(crate) fn execute(command: &str, code: &str, flow: bool) -> LintResponse {
        let result = lint::run(code, flow);
        LintResponse {
            schema_version: SCHEMA_VERSION,
            command: command.into(),
            summary: lint::summarize(&result.violations),
            violations: result.violations,
            flows: if flow { Some(result.flows) } else { None },
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
        let flow = matches.get_flag("flow");
        Ok(Response::Lint(Self::execute(self.name(), &code, flow)))
    }
}
