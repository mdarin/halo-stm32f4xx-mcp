use std::env;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Config {
    pub qdrant_url: String,
    pub qdrant_api_key: Option<String>,
    pub ollama_url: String,
    pub collection: String,
    pub embed_model: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            qdrant_url: env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6333".into()),
            qdrant_api_key: env::var("QDRANT_API_KEY").ok().filter(|k| !k.is_empty()),
            ollama_url: env::var("OLLAMA_URL").unwrap_or_else(|_| "http://localhost:11434".into()),
            collection: env::var("COLLECTION").unwrap_or_else(|_| "stm32f407_docs".into()),
            embed_model: env::var("EMBED_MODEL").unwrap_or_else(|_| "nomic-embed-text".into()),
        }
    }
}
