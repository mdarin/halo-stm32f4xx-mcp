use clap::Command as ClapCommand;

use super::lint::Lint;
use super::Command;
use crate::models::{Response, ToolkitError};

pub struct FlowAnalysis;

impl Command for FlowAnalysis {
    fn name(&self) -> &'static str {
        "flow-analysis"
    }

    fn about(&self) -> &'static str {
        "Same as lint --flow"
    }

    fn clap_command(&self) -> ClapCommand {
        ClapCommand::new(self.name())
            .about(self.about())
            .arg(super::file_flag())
    }

    fn run(&self, matches: &clap::ArgMatches) -> Result<Response, ToolkitError> {
        let code = super::code_from(matches, self.name())?;
        Ok(Response::FlowAnalysis(Lint::execute(
            self.name(),
            &code,
            true,
        )))
    }
}
