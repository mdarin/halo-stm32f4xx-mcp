package main

import (
	"log"
	"os"

	"github.com/mdari/halo-f407-mcp/mcp-server/internal/config"
	"github.com/mdari/halo-f407-mcp/mcp-server/internal/rag/embedder"
	"github.com/mdari/halo-f407-mcp/mcp-server/internal/rag/qdrant"
	"github.com/mdari/halo-f407-mcp/mcp-server/internal/rag/searcher"
	"github.com/mdari/halo-f407-mcp/mcp-server/internal/tools"
	"github.com/mdari/halo-f407-mcp/mcp-server/internal/transport"
)

func main() {
	// Загрузка конфигурации
	cfg := config.Load()
	
	// Логирование в stderr (не мешает STDIO транспорту)
	log.SetOutput(os.Stderr)
	log.Printf("Starting STM32F407 MCP Server...")
	log.Printf("Qdrant: %s", cfg.QdrantURL)
	log.Printf("Ollama: %s", cfg.OllamaURL)
	log.Printf("Collection: %s", cfg.Collection)
	
	// Инициализация компонентов
	qdrantClient := qdrant.NewClient(cfg.QdrantURL)
	embedderClient := embedder.NewClient(cfg.OllamaURL, cfg.Model)
	searcherInstance := searcher.NewSearcher(qdrantClient, embedderClient, cfg.Collection)
	
	// Регистрация инструментов
	registry := tools.NewRegistry()
	registry.Register(tools.NewQueryDocsTool(searcherInstance))
	
	log.Printf("Registered tools: query_hardware_docs")
	
	// Запуск STDIO транспорта
	transport := transport.NewStdioTransport(registry)
	log.Printf("MCP Server ready, listening on stdin...")
	
	if err := transport.Run(); err != nil {
		log.Fatalf("Transport error: %v", err)
	}
}
