package tools

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/mdari/halo-f407-mcp/mcp-server/internal/rag/searcher"
)

type QueryDocsTool struct {
	searcher *searcher.Searcher
}

type QueryDocsArgs struct {
	Query      string `json:"query"`
	Peripheral string `json:"peripheral,omitempty"`
	Limit      int    `json:"limit,omitempty"`
}

func NewQueryDocsTool(searcher *searcher.Searcher) *QueryDocsTool {
	return &QueryDocsTool{
		searcher: searcher,
	}
}

func (t *QueryDocsTool) Name() string {
	return "query_hardware_docs"
}

func (t *QueryDocsTool) Description() string {
	return "Search STM32F407 hardware documentation (RM0090) for registers, peripherals, clock configuration, etc. Returns relevant documentation chunks with metadata."
}

func (t *QueryDocsTool) Execute(ctx context.Context, argsJson json.RawMessage) (interface{}, error) {
	var args QueryDocsArgs
	if err := json.Unmarshal(argsJson, &args); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}
	
	// Установка лимита по умолчанию
	if args.Limit <= 0 {
		args.Limit = 5
	}
	
	// Поиск
	results, err := t.searcher.Search(ctx, args.Query, args.Peripheral, args.Limit)
	if err != nil {
		return nil, fmt.Errorf("search failed: %w", err)
	}
	
	// Форматирование ответа
	response := map[string]interface{}{
		"query":      args.Query,
		"peripheral": args.Peripheral,
		"count":      len(results),
		"results":    results,
	}
	
	return response, nil
}
