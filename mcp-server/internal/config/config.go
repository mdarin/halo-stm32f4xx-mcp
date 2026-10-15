package config

import (
	"os"
)

type Config struct {
	QdrantURL  string
	OllamaURL  string
	Collection string
	Model      string
}

func Load() *Config {
	return &Config{
		QdrantURL:  getEnv("QDRANT_URL", "http://localhost:6333"),
		OllamaURL:  getEnv("OLLAMA_URL", "http://localhost:11434"),
		Collection: getEnv("COLLECTION", "stm32f407_docs"),
		Model:      getEnv("EMBED_MODEL", "nomic-embed-text"),
	}
}

func getEnv(key, defaultValue string) string {
	if value := os.Getenv(key); value != "" {
		return value
	}
	return defaultValue
}
