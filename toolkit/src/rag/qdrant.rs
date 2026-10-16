use crate::config::Config;
use serde::Deserialize;
use std::time::Duration;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct Hit {
    pub id: i64,
    pub score: f64,
    pub payload: serde_json::Map<String, serde_json::Value>,
}

#[derive(Deserialize)]
struct SearchResponse {
    result: Vec<Hit>,
}

pub fn search(
    cfg: &Config,
    vector: &[f32],
    limit: usize,
    filter: Option<&serde_json::Value>,
) -> Result<Vec<Hit>, String> {
    let agent: ureq::Agent = ureq::AgentBuilder::new().timeout(REQUEST_TIMEOUT).build();

    let mut body = serde_json::json!({
        "vector": vector,
        "limit": limit,
        "with_payload": true,
    });
    if let Some(f) = filter {
        body["filter"] = f.clone();
    }

    let mut request = agent
        .post(&format!(
            "{}/collections/{}/points/search",
            cfg.qdrant_url.trim_end_matches('/'),
            cfg.collection
        ))
        .set("Content-Type", "application/json");
    if let Some(key) = &cfg.qdrant_api_key {
        request = request.set("api-key", key);
    }

    let response = request
        .send_json(&body)
        .map_err(|e| format!("qdrant request failed: {e}"))?;

    let parsed: SearchResponse = response
        .into_json()
        .map_err(|e| format!("qdrant decode failed: {e}"))?;

    Ok(parsed.result)
}

#[allow(dead_code)]
pub fn peripheral_filter(peripheral: &str) -> serde_json::Value {
    serde_json::json!({
        "must": [{
            "key": "peripheral",
            "match": { "value": peripheral },
        }],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{body_partial_json, body_string_contains, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn test_cfg(url: &str) -> Config {
        Config {
            qdrant_url: url.into(),
            qdrant_api_key: None,
            ollama_url: "http://unused".into(),
            collection: "test_col".into(),
            embed_model: "m".into(),
        }
    }

    #[tokio::test]
    async fn search_sends_vector_and_parses_hits() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/collections/test_col/points/search"))
            .and(body_partial_json(serde_json::json!({
                "limit": 3,
                "with_payload": true,
            })))
            .and(body_string_contains("\"vector\""))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "result": [
                    {"id": 7, "score": 0.9, "payload": {"text": "DMA docs", "chapter": 10}},
                    {"id": 9, "score": 0.4, "payload": {}}
                ]
            })))
            .mount(&server)
            .await;

        let hits = search(&test_cfg(&server.uri()), &[0.1, 0.2], 3, None).unwrap();
        assert_eq!(hits.len(), 2);
        assert_eq!(hits[0].id, 7);
        assert!((hits[0].score - 0.9).abs() < 1e-9);
        assert_eq!(hits[0].payload["text"], "DMA docs");
    }

    #[tokio::test]
    async fn search_sends_api_key_header_when_configured() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(header("api-key", "secret"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"result": []})),
            )
            .mount(&server)
            .await;
        let mut cfg = test_cfg(&server.uri());
        cfg.qdrant_api_key = Some("secret".into());
        let hits = search(&cfg, &[0.0], 1, None).unwrap();
        assert!(hits.is_empty());
    }

    #[tokio::test]
    async fn search_non_200_is_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        assert!(search(&test_cfg(&server.uri()), &[0.0], 1, None).is_err());
    }

    #[tokio::test]
    async fn search_sends_filter_when_given() {
        let server = MockServer::start().await;
        let filter = peripheral_filter("DMA");
        Mock::given(method("POST"))
            .and(body_partial_json(serde_json::json!({
                "filter": {"must": [{"key": "peripheral", "match": {"value": "DMA"}}]}
            })))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"result": []})),
            )
            .mount(&server)
            .await;
        search(&test_cfg(&server.uri()), &[0.0], 1, Some(&filter)).unwrap();
    }
}
