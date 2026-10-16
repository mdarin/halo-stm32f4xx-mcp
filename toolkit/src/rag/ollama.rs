use crate::config::Config;
use serde::Deserialize;
use std::time::Duration;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);
const RETRY_BACKOFF: [u64; 2] = [1, 2];

#[derive(Deserialize)]
struct EmbedResponse {
    embeddings: Vec<Vec<f32>>,
}

pub async fn embed(client: &reqwest::Client, cfg: &Config, text: &str) -> Result<Vec<f32>, String> {
    let url = format!("{}/api/embed", cfg.ollama_url.trim_end_matches('/'));
    let body = serde_json::json!({
        "model": cfg.embed_model,
        "input": text,
    });

    let mut last_err = String::new();
    for backoff in std::iter::once(0u64).chain(RETRY_BACKOFF.iter().copied()) {
        if backoff > 0 {
            tokio::time::sleep(Duration::from_secs(backoff)).await;
        }
        match embed_once(client, &url, &body).await {
            Ok(v) => return Ok(v),
            Err(e) => last_err = e,
        }
    }
    Err(format!("ollama unavailable after 3 attempts: {last_err}"))
}

async fn embed_once(
    client: &reqwest::Client,
    url: &str,
    body: &serde_json::Value,
) -> Result<Vec<f32>, String> {
    let response = client
        .post(url)
        .timeout(REQUEST_TIMEOUT)
        .json(body)
        .send()
        .await
        .map_err(|e| format!("embed request failed: {e}"))?;

    let parsed: EmbedResponse = response
        .json()
        .await
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

    fn client() -> reqwest::Client {
        reqwest::Client::new()
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

        let v = embed(&client(), &test_cfg(&server.uri()), "hello")
            .await
            .unwrap();
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

        let v = embed(&client(), &test_cfg(&server.uri()), "x")
            .await
            .unwrap();
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

        let err = embed(&client(), &test_cfg(&server.uri()), "x")
            .await
            .unwrap_err();
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
        assert!(embed(&client(), &test_cfg(&server.uri()), "x")
            .await
            .is_err());
    }
}
