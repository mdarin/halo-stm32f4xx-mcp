package main

import (
	"context"
	"fmt"
	"log"
	"os"

	"github.com/mdari/halo-f407-mcp/mcp-server/internal/config"
	"github.com/mdari/halo-f407-mcp/mcp-server/internal/rag/embedder"
	"github.com/mdari/halo-f407-mcp/mcp-server/internal/rag/qdrant"
	"github.com/mdari/halo-f407-mcp/mcp-server/internal/rag/searcher"
	"github.com/modelcontextprotocol/go-sdk/mcp"
)

// QueryDocsInput — входные параметры для query_hardware_docs
type QueryDocsInput struct {
	Query      string `json:"query" jsonschema:"the search query for hardware documentation"`
	Peripheral string `json:"peripheral,omitempty" jsonschema:"optional filter by peripheral type (GPIO, TIM, DMA, etc.)"`
	Limit      int    `json:"limit,omitempty" jsonschema:"optional maximum number of results (default: 5)"`
}

// QueryDocsOutput — результат поиска
type QueryDocsOutput struct {
	Query      string                  `json:"query"`
	Peripheral string                  `json:"peripheral,omitempty"`
	Count      int                     `json:"count"`
	Results    []searcher.SearchResult `json:"results"`
}

// queryDocsHandler — обработчик инструмента query_hardware_docs
func queryDocsHandler(ctx context.Context, req *mcp.CallToolRequest, input QueryDocsInput) (
	*mcp.CallToolResult,
	QueryDocsOutput,
	error,
) {
	// Получение searcher из контекста (или глобальная переменная)
	// Временно используем глобальную переменную
	if globalSearcher == nil {
		return nil, QueryDocsOutput{}, fmt.Errorf("searcher not initialized")
	}

	// Установка лимита по умолчанию
	if input.Limit <= 0 {
		input.Limit = 5
	}

	// Поиск
	results, err := globalSearcher.Search(ctx, input.Query, input.Peripheral, input.Limit)
	if err != nil {
		return nil, QueryDocsOutput{}, fmt.Errorf("search failed: %w", err)
	}

	output := QueryDocsOutput{
		Query:      input.Query,
		Peripheral: input.Peripheral,
		Count:      len(results),
		Results:    results,
	}

	return nil, output, nil
}

// Глобальная переменная для searcher (будет инициализирована в main)
var globalSearcher *searcher.Searcher

func main() {
	// Загрузка конфигурации
	cfg := config.Load()

	// Логирование в stderr (не мешает STDIO транспорту)
	log.SetOutput(os.Stderr)
	log.Printf("Starting STM32F407 MCP Server with official MCP SDK v1.8.0...")
	log.Printf("Qdrant: %s", cfg.QdrantURL)
	log.Printf("Ollama: %s", cfg.OllamaURL)
	log.Printf("Collection: %s", cfg.Collection)

	// Инициализация компонентов
	qdrantClient := qdrant.NewClient(cfg.QdrantURL)
	embedderClient := embedder.NewClient(cfg.OllamaURL, cfg.Model)
	globalSearcher = searcher.NewSearcher(qdrantClient, embedderClient, cfg.Collection)

	// Создание MCP сервера
	server := mcp.NewServer(
		&mcp.Implementation{
			Name:    "stm32f407-mcp",
			Version: "0.1.0",
		},
		nil, // options
	)

	// Регистрация инструмента query_hardware_docs
	mcp.AddTool(
		server,
		&mcp.Tool{
			Name:        "query_hardware_docs",
			Description: "Search STM32F407 hardware documentation (RM0090) for registers, peripherals, clock configuration, etc. Returns relevant documentation chunks with metadata.",
		},
		queryDocsHandler,
	)

	log.Printf("Registered tools: query_hardware_docs")
	log.Printf("MCP Server ready, listening on stdin...")

	// Запуск STDIO транспорта
	if err := server.Run(context.Background(), &mcp.StdioTransport{}); err != nil {
		log.Fatalf("Server error: %v", err)
	}
}
