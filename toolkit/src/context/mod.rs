//! Контекстная память агента (Фаза 5, CONTRACT.md §10).
//!
//! Отдельная коллекция Qdrant `stm32_project_context`: периферия, IRQ,
//! история модификаций, пользовательские предпочтения. Go mcp-server
//! прямого доступа к коллекции не имеет — только exec toolkit.
// Схема попадает в CLI ctx-команды в Ф5-2; до тех пор часть API не вызывается.
#![allow(dead_code)]

pub const CONTEXT_COLLECTION_DEFAULT: &str = "stm32_project_context";

pub const DOC_TYPE_CONTEXT: &str = "project_context";
pub const DOC_TYPE_MODIFICATION: &str = "modification";
pub const DOC_TYPE_PREFERENCE: &str = "preference";

/// Контекст проекта: payload точки в `stm32_project_context` (CONTRACT §10).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProjectContext {
    /// Абсолютный путь к проекту — основное filter-поле.
    #[serde(rename = "project_path")]
    pub project_path: String,
    /// Логический ключ записи: "summary", "prefs", "mod:<file>:<block>:<ts>"...
    #[serde(rename = "ctx_key")]
    pub key: String,
    /// doc_type: project_context | modification | preference
    #[serde(rename = "doc_type")]
    pub doc_type: String,
    /// Свободный текст (эмбеддится).
    pub text: String,
    /// Периферия: ["GPIO", "TIM3", "ADC1", "USART2"].
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub peripherals: Vec<String>,
    /// IRQ-обработчики: ["TIM3_IRQHandler", "USART2_IRQHandler"].
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interrupts: Vec<String>,
    /// Предпочтения пользователя: {"style": "doxygen", ...}.
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub user_preferences: std::collections::HashMap<String, String>,
    /// История модификации (для doc_type=modification).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modification: Option<Modification>,
    #[serde(default)]
    pub schema_version: u32,
}

/// Запись об изменении, внесённом агентом (CONTRACT §10).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Modification {
    /// Файл, в который вносилась правка.
    pub file: String,
    /// Имя USER CODE блока ("Includes", "PV", "4"...).
    pub block: String,
    /// Режим вставки: append | replace.
    pub mode: String,
    /// Байт записано.
    pub bytes: usize,
    /// Unix-время операции (сек).
    pub timestamp: i64,
}

/// Детерминированный id точки — первые 8 hex SHA-256 от
/// "{project_path}:{key}"; повторный ctx-save перезаписывает точку.
pub fn context_id(project_path: &str, key: &str) -> i64 {
    use sha2::{Digest, Sha256};
    let input = format!("{project_path}:{key}");
    let digest = Sha256::digest(input.as_bytes());
    let hex: String = digest
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<String>()[..8]
        .to_string();
    i64::from(u32::from_str_radix(&hex, 16).unwrap_or(0))
}

impl ProjectContext {
    /// Готовая точка Qdrant (id, payload) — вектор добавляет вызывающая
    /// сторона после эмбеддинга text.
    pub fn to_point(&self) -> serde_json::Value {
        let payload = serde_json::json!({
            "project_path": self.project_path,
            "ctx_key": self.key,
            "doc_type": self.doc_type,
            "text": self.text,
            "peripherals": self.peripherals,
            "interrupts": self.interrupts,
            "user_preferences": self.user_preferences,
            "modification": self.modification,
            "schema_version": self.schema_version,
        });
        serde_json::json!({
            "id": context_id(&self.project_path, &self.key),
            "payload": payload,
        })
    }
}

/// Имя коллекции контекста: env `CONTEXT_COLLECTION`, дефолт
/// `stm32_project_context` (НЕ общий `COLLECTION` документации).
pub fn collection_from_env() -> String {
    std::env::var("CONTEXT_COLLECTION")
        .ok()
        .filter(|c| !c.is_empty())
        .unwrap_or_else(|| CONTEXT_COLLECTION_DEFAULT.to_string())
}

/// Фильтр Qdrant по проекту.
pub fn project_filter(project_path: &str) -> serde_json::Value {
    serde_json::json!({
        "must": [{
            "key": "project_path",
            "match": { "value": project_path },
        }],
    })
}

/// Config с коллекцией контекста (общие URL/ключ/модель — из env).
fn ctx_cfg(cfg: &crate::config::Config, collection: &str) -> crate::config::Config {
    let mut c = cfg.clone();
    c.collection = collection.to_string();
    c
}

/// ctx-save: эмбеддинг text каждой записи и upsert в коллекцию контекста.
/// Коллекция создаётся при отсутствии. Возвращает число сохранённых точек.
pub fn save(
    cfg: &crate::config::Config,
    collection: &str,
    entries: Vec<ProjectContext>,
) -> Result<usize, String> {
    use crate::rag::ollama;
    if entries.is_empty() {
        return Ok(0);
    }
    ensure_collection(&cfg.qdrant_url, cfg.qdrant_api_key.as_deref(), collection)?;
    let c = ctx_cfg(cfg, collection);
    let mut points = Vec::with_capacity(entries.len());
    for e in &entries {
        let mut point = e.to_point();
        point["vector"] = serde_json::json!(ollama::embed(&c, &e.text)?);
        points.push(point);
    }
    crate::rag::qdrant::upsert(&c, &points)?;
    Ok(points.len())
}

/// ctx-load: чтение payload точек проекта (без эмбеддинга).
/// key = Some — точечная выборка одного ключа, None — весь проект.
pub fn load(
    cfg: &crate::config::Config,
    collection: &str,
    project_path: &str,
    key: Option<&str>,
) -> Result<Vec<crate::models::CtxEntry>, String> {
    let agent: ureq::Agent = ureq::AgentBuilder::new()
        .timeout(std::time::Duration::from_secs(30))
        .build();
    let mut filter = project_filter(project_path);
    if let Some(k) = key {
        filter["must"]
            .as_array_mut()
            .unwrap()
            .push(serde_json::json!({
                "key": "ctx_key",
                "match": { "value": k },
            }));
    }
    let body = serde_json::json!({
        "filter": filter,
        "limit": 1000,
        "with_payload": true,
    });
    let mut req = agent.post(&format!(
        "{}/collections/{}/points/scroll",
        cfg.qdrant_url.trim_end_matches('/'),
        collection
    ));
    if let Some(api) = &cfg.qdrant_api_key {
        req = req.set("api-key", api);
    }
    let resp = req
        .send_json(&body)
        .map_err(|e| format!("qdrant request failed: {e}"))?;
    let parsed: serde_json::Value = resp
        .into_json()
        .map_err(|e| format!("qdrant decode failed: {e}"))?;

    let mut entries = Vec::new();
    for p in parsed["result"]["points"]
        .as_array()
        .cloned()
        .unwrap_or_default()
    {
        let pl = &p["payload"];
        entries.push(crate::models::CtxEntry {
            ctx_key: pl["ctx_key"].as_str().unwrap_or_default().into(),
            doc_type: pl["doc_type"].as_str().unwrap_or_default().into(),
            text: pl["text"].as_str().unwrap_or_default().into(),
            peripherals: str_vec(&pl["peripherals"]),
            interrupts: str_vec(&pl["interrupts"]),
            user_preferences: prefs_map(&pl["user_preferences"]),
            modification: pl
                .get("modification")
                .filter(|m| !m.is_null())
                .and_then(|m| serde_json::from_value(m.clone()).ok()),
            timestamp: pl["modification"]["timestamp"].as_i64(),
        });
    }
    entries.sort_by_key(|e| e.timestamp.unwrap_or(0));
    Ok(entries)
}

/// ctx-search: семантический поиск по контексту проекта.
pub fn search(
    cfg: &crate::config::Config,
    collection: &str,
    project_path: &str,
    query: &str,
    limit: usize,
) -> Result<Vec<crate::models::CtxHit>, String> {
    use crate::rag::ollama;
    let c = ctx_cfg(cfg, collection);
    let vector = ollama::embed(&c, query)?;
    let filter = project_filter(project_path);
    let hits = crate::rag::qdrant::search(&c, &vector, limit, Some(&filter))?;
    Ok(hits
        .into_iter()
        .map(|hit| {
            let pl = &hit.payload;
            crate::models::CtxHit {
                score: hit.score,
                project_path: pl["project_path"].as_str().unwrap_or_default().into(),
                ctx_key: pl["ctx_key"].as_str().unwrap_or_default().into(),
                doc_type: pl["doc_type"].as_str().unwrap_or_default().into(),
                text: pl["text"].as_str().unwrap_or_default().into(),
            }
        })
        .collect())
}

fn str_vec(v: &serde_json::Value) -> Vec<String> {
    v.as_array()
        .map(|a| {
            a.iter()
                .filter_map(|x| x.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default()
}

fn prefs_map(v: &serde_json::Value) -> std::collections::HashMap<String, String> {
    v.as_object()
        .map(|o| {
            o.iter()
                .filter_map(|(k, x)| x.as_str().map(|s| (k.clone(), s.to_string())))
                .collect()
        })
        .unwrap_or_default()
}

/// Создание коллекции контекста, если её нет (768 dim Cosine + bm25 —
/// та же геометрия, что у документационной коллекции). Идемпотентно.
pub fn ensure_collection(
    qdrant_url: &str,
    api_key: Option<&str>,
    collection: &str,
) -> Result<(), String> {
    let agent: ureq::Agent = ureq::AgentBuilder::new()
        .timeout(std::time::Duration::from_secs(30))
        .build();
    let base = qdrant_url.trim_end_matches('/');

    let mut req = agent.get(&format!("{base}/collections"));
    if let Some(k) = api_key {
        req = req.set("api-key", k);
    }
    let resp = req
        .call()
        .map_err(|e| format!("qdrant request failed: {e}"))?;
    let listed: serde_json::Value = resp
        .into_json()
        .map_err(|e| format!("qdrant decode failed: {e}"))?;
    let exists = listed["result"]["collections"]
        .as_array()
        .map(|cs| cs.iter().any(|c| c["name"].as_str() == Some(collection)))
        .unwrap_or(false);
    if exists {
        return Ok(());
    }

    let body = serde_json::json!({
        "vectors": {"size": 768, "distance": "Cosine"},
        "sparse_vectors": {"bm25": {}},
    });
    let mut req = agent.put(&format!("{base}/collections/{collection}"));
    if let Some(k) = api_key {
        req = req.set("api-key", k);
    }
    let resp = req
        .send_json(&body)
        .map_err(|e| format!("qdrant create collection failed: {e}"))?;
    if resp.status() != 200 {
        return Err(format!(
            "qdrant create collection returned status {}",
            resp.status()
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_id_is_deterministic_and_distinct() {
        let a = context_id("/p1", "summary");
        assert_eq!(a, context_id("/p1", "summary"));
        assert_ne!(a, context_id("/p2", "summary"));
        assert_ne!(a, context_id("/p1", "prefs"));
    }

    #[test]
    fn context_id_matches_python_sha256_parity() {
        // echo -n "/p1:summary" | sha256sum | cut -c1-8 → 0x... u32
        let expected: i64 = {
            use sha2::{Digest, Sha256};
            let d = Sha256::digest(b"/p1:summary");
            let hex: String =
                d.iter().map(|b| format!("{b:02x}")).collect::<String>()[..8].to_string();
            i64::from(u32::from_str_radix(&hex, 16).unwrap())
        };
        assert_eq!(context_id("/p1", "summary"), expected);
    }

    #[test]
    fn to_point_embeds_payload_and_id() {
        let ctx = ProjectContext {
            project_path: "/p".into(),
            key: "summary".into(),
            doc_type: DOC_TYPE_CONTEXT.into(),
            text: "TIM3 is PWM".into(),
            peripherals: vec!["TIM3".into()],
            interrupts: vec![],
            user_preferences: std::collections::HashMap::new(),
            modification: None,
            schema_version: 1,
        };
        let point = ctx.to_point();
        assert_eq!(point["id"], context_id("/p", "summary"));
        assert_eq!(point["payload"]["doc_type"], DOC_TYPE_CONTEXT);
        assert_eq!(point["payload"]["text"], "TIM3 is PWM");
        assert_eq!(point["payload"]["peripherals"][0], "TIM3");
        // Пустые поля сериализуются как пустые значения
        assert_eq!(point["payload"]["interrupts"], serde_json::json!([]));
        assert_eq!(point["payload"]["user_preferences"], serde_json::json!({}));
    }

    #[test]
    fn modification_roundtrip() {
        let m = Modification {
            file: "src/main.c".into(),
            block: "4".into(),
            mode: "append".into(),
            bytes: 128,
            timestamp: 1_760_000_000,
        };
        let json = serde_json::to_string(&m).unwrap();
        let back: Modification = serde_json::from_str(&json).unwrap();
        assert_eq!(m, back);
    }

    #[test]
    fn project_filter_matches_value() {
        let f = project_filter("/p");
        assert_eq!(f["must"][0]["key"], "project_path");
        assert_eq!(f["must"][0]["match"]["value"], "/p");
    }

    #[tokio::test]
    async fn ensure_collection_creates_when_missing() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/collections"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "result": {"collections": [{"name": "other"}]}
            })))
            .mount(&server)
            .await;
        Mock::given(method("PUT"))
            .and(path("/collections/ctx_col"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "result": {"ok": true}
            })))
            .mount(&server)
            .await;
        ensure_collection(&server.uri(), None, "ctx_col").unwrap();
    }

    #[tokio::test]
    async fn ensure_collection_idempotent_when_exists() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/collections"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "result": {"collections": [{"name": "ctx_col"}]}
            })))
            .mount(&server)
            .await;
        // PUT не замаунтен: любой вызов упал бы с 404 у wiremock → ошибка.
        ensure_collection(&server.uri(), None, "ctx_col").unwrap();
    }

    #[tokio::test]
    async fn ensure_collection_error_on_500() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/collections"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        assert!(ensure_collection(&server.uri(), None, "ctx_col").is_err());
    }
}
