use crate::config::Config;
use serde::Deserialize;
use std::thread::sleep;
use std::time::Duration;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);
const RETRY_BACKOFF: [Duration; 2] = [Duration::from_secs(1), Duration::from_secs(2)];

#[derive(Deserialize)]
struct EmbedResponse {
    embeddings: Vec<Vec<f32>>,
}

pub fn embed(cfg: &Config, text: &str) -> Result<Vec<f32>, String> {
    let agent: ureq::Agent = ureq::AgentBuilder::new().timeout(REQUEST_TIMEOUT).build();

    let body = serde_json::json!({
        "model": cfg.embed_model,
        "input": text,
    });

    let mut last_err = String::new();
    for (attempt, backoff) in std::iter::once(&Duration::ZERO)
        .chain(RETRY_BACKOFF.iter())
        .enumerate()
    {
        if !backoff.is_zero() {
            sleep(*backoff);
        }
        match embed_once(&agent, cfg, &body) {
            Ok(v) => return Ok(v),
            Err(e) => last_err = e,
        }
        let _ = attempt;
    }
    Err(format!("ollama unavailable after 3 attempts: {last_err}"))
}

fn embed_once(
    agent: &ureq::Agent,
    cfg: &Config,
    body: &serde_json::Value,
) -> Result<Vec<f32>, String> {
    let response = agent
        .post(&format!(
            "{}/api/embed",
            cfg.ollama_url.trim_end_matches('/')
        ))
        .set("Content-Type", "application/json")
        .send_json(body)
        .map_err(|e| format!("embed request failed: {e}"))?;

    let parsed: EmbedResponse = response
        .into_json()
        .map_err(|e| format!("embed decode failed: {e}"))?;

    parsed
        .embeddings
        .into_iter()
        .next()
        .ok_or_else(|| "empty embedding returned".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn test_cfg(url: &str) -> Config {
        Config {
            qdrant_url: "http://unused".into(),
            qdrant_api_key: None,
            ollama_url: url.into(),
            collection: "c".into(),
            embed_model: "test-model".into(),
        }
    }

    #[tokio::test]
    async fn embed_sends_model_and_input() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/embed"))
            .and(body_json(
                serde_json::json!({"model": "test-model", "input": "hello"}),
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "embeddings": [[0.5, -0.5]]
            })))
            .mount(&server)
            .await;

        let v = embed(&test_cfg(&server.uri()), "hello").unwrap();
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], 0.5);
    }

    #[tokio::test]
    async fn embed_retries_then_succeeds() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(500))
            .up_to_n_times(2)
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "embeddings": [[1.0]]
            })))
            .mount(&server)
            .await;

        let v = embed(&test_cfg(&server.uri()), "x").unwrap();
        assert_eq!(v, vec![1.0]);
    }

    #[tokio::test]
    async fn embed_fails_after_three_attempts() {
        let server = MockServer::start().await;
        let expectation = Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(500))
            .expect(3)
            .mount_as_scoped(&server)
            .await;

        let err = embed(&test_cfg(&server.uri()), "x").unwrap_err();
        assert!(err.contains("after 3 attempts"), "{err}");
        drop(expectation);
    }

    #[tokio::test]
    async fn embed_empty_embeddings_is_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "embeddings": []
            })))
            .mount(&server)
            .await;
        assert!(embed(&test_cfg(&server.uri()), "x").is_err());
    }
}
