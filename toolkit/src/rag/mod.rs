pub mod ollama;
pub mod qdrant;

use crate::config::Config;
use crate::lint;
use crate::models::{RagRef, RagResponse, SCHEMA_VERSION};

const SNIPPET_MAX_CHARS: usize = 300;

pub fn run(code: &str, rule_context: &str, limit: usize, cfg: &Config) -> RagResponse {
    let violations = lint::run(code, false).violations;

    let query = if rule_context.trim().is_empty() {
        &code[..code.floor_char_boundary(code.len().min(2000))]
    } else {
        rule_context
    };

    let mut rag_refs = Vec::new();
    let mut rag_degraded = false;

    match fetch_refs(cfg, query, limit) {
        Ok(refs) => rag_refs = refs,
        Err(e) => {
            rag_degraded = true;
            eprintln!(
                "{}",
                serde_json::json!({
                    "error": {
                        "code": "E_RAG_UNAVAILABLE",
                        "message": e,
                        "command": "rag-search",
                    }
                })
            );
        }
    }

    RagResponse {
        schema_version: SCHEMA_VERSION,
        command: "rag-search".into(),
        violations,
        rag_refs,
        rag_degraded,
    }
}

fn fetch_refs(cfg: &Config, query: &str, limit: usize) -> Result<Vec<RagRef>, String> {
    let vector = ollama::embed(cfg, query)?;
    let hits = qdrant::search(cfg, &vector, limit, None)?;
    Ok(hits
        .into_iter()
        .map(|hit| {
            let payload = hit.payload;
            let text = payload
                .get("text")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            let snippet: String = text.chars().take(SNIPPET_MAX_CHARS).collect();
            RagRef {
                score: hit.score,
                source: payload
                    .get("source_file")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string(),
                title: payload
                    .get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string(),
                chapter: payload
                    .get("chapter")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0) as u32,
                section: payload
                    .get("section")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string(),
                snippet,
            }
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{body_partial_json, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn test_cfg(ollama: &str, qdrant: &str) -> Config {
        Config {
            qdrant_url: qdrant.into(),
            qdrant_api_key: None,
            ollama_url: ollama.into(),
            collection: "c".into(),
            embed_model: "m".into(),
        }
    }

    fn tests_code_with_ccm_dma() -> String {
        std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/ccm_dma.c"
        ))
        .unwrap()
    }

    #[tokio::test]
    async fn returns_violations_and_refs_when_rag_alive() {
        let ollama_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/embed"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "embeddings": [[0.25, 0.75]]
            })))
            .mount(&ollama_server)
            .await;

        let qdrant_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/collections/c/points/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "result": [{
                    "id": 1,
                    "score": 0.73,
                    "payload": {
                        "text": "RM0090 DMA chunk",
                        "source_file": "10_10_3_DMA.md",
                        "title": "DMA functional description",
                        "chapter": 10,
                        "section": "10.3"
                    }
                }]
            })))
            .mount(&qdrant_server)
            .await;

        let code = tests_code_with_ccm_dma();
        let r = run(
            &code,
            "",
            5,
            &test_cfg(&ollama_server.uri(), &qdrant_server.uri()),
        );
        assert!(!r.rag_degraded);
        assert_eq!(r.violations.len(), 1);
        assert_eq!(r.rag_refs.len(), 1);
        assert_eq!(r.rag_refs[0].source, "10_10_3_DMA.md");
        assert_eq!(r.rag_refs[0].chapter, 10);
        assert_eq!(r.rag_refs[0].section, "10.3");
        assert!((r.rag_refs[0].score - 0.73).abs() < 1e-9);
    }

    #[tokio::test]
    async fn degrades_when_rag_unreachable() {
        let cfg = test_cfg("http://127.0.0.1:1", "http://127.0.0.1:1");
        let code = tests_code_with_ccm_dma();
        let r = run(&code, "", 5, &cfg);
        assert!(r.rag_degraded);
        assert_eq!(r.rag_refs, Vec::new());
        assert_eq!(r.violations.len(), 1, "violations survive degradation");
    }

    #[tokio::test]
    async fn rule_context_is_embedded_instead_of_code() {
        let ollama_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(body_partial_json(
                serde_json::json!({"input": "DMA buffer placement"}),
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "embeddings": [[1.0]]
            })))
            .mount(&ollama_server)
            .await;
        let qdrant_server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"result": []})),
            )
            .mount(&qdrant_server)
            .await;

        let r = run(
            "int main(void) { return 0; }\n",
            "DMA buffer placement",
            5,
            &test_cfg(&ollama_server.uri(), &qdrant_server.uri()),
        );
        assert!(!r.rag_degraded);
        assert!(r.rag_refs.is_empty());
    }
}
