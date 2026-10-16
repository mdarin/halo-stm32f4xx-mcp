pub mod ollama;
pub mod qdrant;

use crate::config::Config;
use crate::lint;
use crate::models::{RagRef, RagResponse, SCHEMA_VERSION};
use reqwest::Client;

const SNIPPET_MAX_CHARS: usize = 300;

/// Runtime для RAG-команд (CLI одноразовый, неблочные команды tokio не трогают).
pub fn rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("tokio runtime")
}

pub async fn run(code: &str, rule_context: &str, limit: usize, cfg: &Config) -> RagResponse {
    let client = Client::new();

    let query = if rule_context.trim().is_empty() {
        &code[..code.floor_char_boundary(code.len().min(2000))]
    } else {
        rule_context
    };

    // lint (CPU) и embed (сеть) независимо — гоняем параллельно.
    let code_owned = code.to_string();
    let (lint_res, refs_res) = tokio::join!(
        tokio::task::spawn_blocking(move || lint::run(&code_owned, false).violations),
        fetch_refs(&client, cfg, query, limit),
    );

    let violations = lint_res.unwrap_or_default();
    let mut rag_refs = Vec::new();
    let mut rag_degraded = false;

    match refs_res {
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

async fn fetch_refs(
    client: &Client,
    cfg: &Config,
    query: &str,
    limit: usize,
) -> Result<Vec<RagRef>, String> {
    let vector = ollama::embed(client, cfg, query).await?;
    let hits = qdrant::search(client, cfg, &vector, limit, None).await?;
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

// ============================================================================
// rag-query / rag-upsert / rag-index (CONTRACT.md §6–7, Ф2-1)
// ============================================================================

/// Точка для upsert: детерминированный id, текст для эмбеддинга, payload.
#[derive(Debug, Clone)]
pub struct UpsertPoint {
    pub id: i64,
    pub text: String,
    pub payload: serde_json::Value,
}

/// Поиск по документации: embed(query) → Qdrant search (+фильтр peripheral).
pub async fn query(
    text: &str,
    peripheral: &str,
    limit: usize,
    cfg: &Config,
) -> Result<Vec<crate::models::DocHit>, String> {
    let client = Client::new();
    let vector = ollama::embed(&client, cfg, text).await?;
    let filter = if peripheral.trim().is_empty() {
        None
    } else {
        Some(qdrant::peripheral_filter(peripheral))
    };
    let hits = qdrant::search(&client, cfg, &vector, limit, filter.as_ref()).await?;
    Ok(hits.into_iter().map(hit_to_doc).collect())
}

fn hit_to_doc(hit: qdrant::Hit) -> crate::models::DocHit {
    let p = hit.payload;
    crate::models::DocHit {
        text: p
            .get("text")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .into(),
        source: p
            .get("source_file")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .into(),
        peripheral: p
            .get("peripheral")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .into(),
        chapter: p.get("chapter").and_then(|v| v.as_i64()).unwrap_or(0),
        section: p
            .get("section")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .into(),
        title: p
            .get("title")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .into(),
        score: hit.score,
    }
}

/// Детерминированный id точки — parity с scripts/index_docs.py:
/// первые 8 hex-символов SHA-256 от "{file}:{chunk_idx}".
pub fn deterministic_id(file: &str, chunk_idx: usize) -> i64 {
    use sha2::{Digest, Sha256};
    let input = format!("{file}:{chunk_idx}");
    let digest = Sha256::digest(input.as_bytes());
    let hex: String = digest
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<String>()[..8]
        .to_string();
    i64::from(u32::from_str_radix(&hex, 16).unwrap_or(0))
}

const EMBED_CONCURRENCY: usize = 8;
const BATCH: usize = 100;

/// Эмбеддинг группы текстов параллельно, порядок результатов = порядку входа.
async fn embed_all(
    client: &Client,
    cfg: &Config,
    texts: &[String],
) -> Vec<Result<Vec<f32>, String>> {
    let mut results = Vec::with_capacity(texts.len());
    for group in texts.chunks(EMBED_CONCURRENCY) {
        let handles: Vec<_> = group
            .iter()
            .map(|t| {
                let text = t.clone();
                let client = client.clone();
                let cfg = cfg.clone();
                tokio::spawn(async move { ollama::embed(&client, &cfg, &text).await })
            })
            .collect();
        for handle in handles {
            results.push(
                handle
                    .await
                    .unwrap_or_else(|e| Err(format!("embed task failed: {e}"))),
            );
        }
    }
    results
}

/// Upsert точек: текст каждой точки эмбеддится параллельно, точки грузятся батчами по 100.
pub async fn upsert_points(points: Vec<UpsertPoint>, cfg: &Config) -> Result<usize, String> {
    let total = points.len();
    let client = Client::new();

    let texts: Vec<String> = points.iter().map(|p| p.text.clone()).collect();
    let vectors = embed_all(&client, cfg, &texts).await;

    let mut qdrant_points = Vec::with_capacity(points.len());
    for (point, vec_res) in points.into_iter().zip(vectors) {
        let vector = vec_res?;
        let mut payload = point.payload.clone();
        if !payload.is_object() {
            payload = serde_json::json!({});
        }
        payload["text"] = serde_json::Value::String(point.text.clone());
        qdrant_points.push(serde_json::json!({
            "id": point.id,
            "vector": vector,
            "payload": payload,
        }));
        if qdrant_points.len() == BATCH {
            qdrant::upsert(&client, cfg, &qdrant_points).await?;
            qdrant_points.clear();
        }
    }
    qdrant::upsert(&client, cfg, &qdrant_points).await?;
    Ok(total)
}

/// Статистика индексации (CONTRACT.md §7).
#[derive(Debug, Default, Clone)]
pub struct IndexStats {
    pub files: usize,
    pub chunks: usize,
    pub skipped: usize,
    pub errors: usize,
}

const LARGE_FILE_CHARS: usize = 30_000;
const MIN_CHUNK_CHARS: usize = 50;

/// Индексация kb-каталога по manifest.json (parity со scripts/index_docs.py):
/// маленькие файлы — одна точка, большие (>30000) — чанки по заголовкам "### ".
/// Id детерминированные (deterministic_id), повторный запуск перезаписывает точки.
pub async fn index_kb(dir: &std::path::Path, cfg: &Config) -> Result<IndexStats, String> {
    let manifest_path = dir.join("manifest.json");
    let manifest_raw = std::fs::read_to_string(&manifest_path)
        .map_err(|e| format!("failed to read {}: {e}", manifest_path.display()))?;
    let manifest: serde_json::Value =
        serde_json::from_str(&manifest_raw).map_err(|e| format!("manifest.json: {e}"))?;
    let files = manifest
        .get("files")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "manifest.json: missing files array".to_string())?;

    let mut stats = IndexStats::default();
    for entry in files {
        if entry
            .get("not_on_f407")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            stats.skipped += 1;
            continue;
        }
        let file_name = match entry.get("file").and_then(|v| v.as_str()) {
            Some(f) => f.to_string(),
            None => {
                stats.errors += 1;
                continue;
            }
        };
        let content = match std::fs::read_to_string(dir.join(&file_name)) {
            Ok(c) => c,
            Err(_) => {
                stats.errors += 1;
                continue;
            }
        };
        if content.trim().chars().count() < MIN_CHUNK_CHARS {
            stats.skipped += 1;
            continue;
        }

        let title = entry
            .get("title")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        let metadata = serde_json::json!({
            "source_file": file_name,
            "chapter": entry.get("chapter"),
            "section": entry.get("section"),
            "title": entry.get("title"),
            "peripheral": entry.get("peripheral"),
            "registers": entry.get("registers").cloned().unwrap_or(serde_json::json!([])),
            "tags": entry.get("tags").cloned().unwrap_or(serde_json::json!([])),
            "not_on_f407": false,
            "schema_version": 1,
        });

        let mut points = Vec::new();
        if content.chars().count() > LARGE_FILE_CHARS {
            for (i, (header, chunk)) in chunk_by_headers(&content).into_iter().enumerate() {
                if chunk.trim().chars().count() < MIN_CHUNK_CHARS {
                    continue;
                }
                let full_text = format!("{title}\n{header}\n{chunk}");
                let mut payload = metadata.clone();
                payload["chunk_id"] = serde_json::json!(i);
                payload["chunk_header"] = serde_json::json!(header);
                points.push(UpsertPoint {
                    id: deterministic_id(&file_name, i),
                    text: full_text,
                    payload,
                });
            }
        } else {
            let mut payload = metadata.clone();
            payload["chunk_id"] = serde_json::json!(0);
            points.push(UpsertPoint {
                id: deterministic_id(&file_name, 0),
                text: content.clone(),
                payload,
            });
        }

        let count = points.len();
        match upsert_points(points, cfg).await {
            Ok(_) => {
                stats.files += 1;
                stats.chunks += count;
            }
            Err(_) => {
                stats.errors += 1;
            }
        }
    }
    Ok(stats)
}

/// Чанкинг по заголовкам "### " (parity с chunk_by_headers в index_docs.py).
fn chunk_by_headers(content: &str) -> Vec<(String, String)> {
    let mut chunks = Vec::new();
    let mut current: Vec<&str> = Vec::new();
    let mut header = String::new();

    for line in content.lines() {
        if line.starts_with("### ") {
            if !current.is_empty() {
                chunks.push((header.clone(), current.join("\n")));
            }
            header = line.to_string();
            current = vec![line];
        } else {
            current.push(line);
        }
    }
    if !current.is_empty() {
        chunks.push((header, current.join("\n")));
    }
    chunks
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
        )
        .await;
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
        let r = run(&code, "", 5, &cfg).await;
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
        )
        .await;
        assert!(!r.rag_degraded);
        assert!(r.rag_refs.is_empty());
    }
}
