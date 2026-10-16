mod analyzer;
mod commands;
mod config;
mod docs_gen;
mod lint;
mod models;
mod rag;

use std::io::Write;
use std::process::ExitCode;

use models::ToolkitError;

fn main() -> ExitCode {
    let cmds = commands::registry();
    let root = commands::build_root(&cmds);
    let matches = root.get_matches();
    match commands::dispatch(&cmds, &matches) {
        Ok(response) => {
            let json = serde_json::to_string(&response).expect("response serialization");
            let mut stdout = std::io::stdout().lock();
            if stdout
                .write_all(json.as_bytes())
                .and_then(|_| stdout.write_all(b"\n"))
                .is_err()
            {
                let err =
                    ToolkitError::new(models::ErrorCode::Internal, "failed to write stdout", "");
                eprintln!("{}", err.to_json());
                return ExitCode::from(1);
            }
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{}", err.to_json());
            ExitCode::from(1)
        }
    }
}
