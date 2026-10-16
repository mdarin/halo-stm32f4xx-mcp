use clap::Command as ClapCommand;
use serde::Deserialize;

use super::Command;
use crate::config::Config;
use crate::models::{RagUpsertResponse, Response, SCHEMA_VERSION};
use crate::rag;

#[derive(Deserialize)]
struct UpsertRequest {
    points: Vec<UpsertPointInput>,
}

#[derive(Deserialize)]
struct UpsertPointInput {
    id: i64,
    text: String,
    #[serde(default)]
    payload: serde_json::Value,
}

pub struct RagUpsert;

impl Command for RagUpsert {
    fn name(&self) -> &'static str {
        "rag-upsert"
    }

    fn about(&self) -> &'static str {
        "Embed and upsert points into the documentation knowledge base (stdin JSON)"
    }

    fn clap_command(&self) -> ClapCommand {
        ClapCommand::new(self.name())
            .about(self.about())
            .after_help(
                "stdin JSON: {\"points\": [{\"id\": 42, \"text\": \"...\", \"payload\": {...}}]}\n\
                 Ids are caller-chosen; text is embedded, payload is merged with \"text\".",
            )
    }

    fn run(&self, _matches: &clap::ArgMatches) -> Result<Response, crate::models::ToolkitError> {
        let input = super::read_stdin(self.name())?;
        let request: UpsertRequest = serde_json::from_str(&input).map_err(|e| {
            crate::models::ToolkitError::new(
                crate::models::ErrorCode::InputRead,
                format!("invalid upsert request JSON: {e}"),
                self.name(),
            )
        })?;
        let cfg = Config::from_env();

        let points: Vec<rag::UpsertPoint> = request
            .points
            .into_iter()
            .map(|p| rag::UpsertPoint {
                id: p.id,
                text: p.text,
                payload: p.payload,
            })
            .collect();

        let upserted = rag::upsert_points(points, &cfg).map_err(|e| {
            crate::models::ToolkitError::new(
                crate::models::ErrorCode::RagUnavailable,
                e,
                self.name(),
            )
        })?;

        Ok(Response::RagUpsert(RagUpsertResponse {
            schema_version: SCHEMA_VERSION,
            command: self.name().into(),
            upserted,
        }))
    }
}
