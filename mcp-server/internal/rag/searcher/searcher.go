package searcher

import (
	"context"
	"fmt"

	"github.com/mdari/halo-f407-mcp/mcp-server/internal/rag/embedder"
	"github.com/mdari/halo-f407-mcp/mcp-server/internal/rag/qdrant"
)

type Searcher struct {
	qdrant    *qdrant.Client
	embedder  *embedder.Client
	collection string
}

type SearchResult struct {
	Text       string  `json:"text"`
	Source     string  `json:"source"`
	Peripheral string  `json:"peripheral"`
	Chapter    int     `json:"chapter"`
	Section    string  `json:"section"`
	Title      string  `json:"title"`
	Score      float32 `json:"score"`
}

func NewSearcher(qdrantClient *qdrant.Client, embedderClient *embedder.Client, collection string) *Searcher {
	return &Searcher{
		qdrant:     qdrantClient,
		embedder:   embedderClient,
		collection: collection,
	}
}

func (s *Searcher) Search(ctx context.Context, query string, peripheral string, limit int) ([]SearchResult, error) {
	// Получение эмбеддинга для запроса
	vector, err := s.embedder.Embed(ctx, query)
	if err != nil {
		return nil, fmt.Errorf("embed query: %w", err)
	}
	
	// Построение фильтра
	var filter map[string]interface{}
	if peripheral != "" {
		filter = map[string]interface{}{
			"must": []map[string]interface{}{
				{
					"key": "peripheral",
					"match": map[string]interface{}{
						"value": peripheral,
					},
				},
			},
		}
	}
	
	// Поиск в Qdrant
	hits, err := s.qdrant.Search(ctx, s.collection, vector, limit, filter)
	if err != nil {
		return nil, fmt.Errorf("qdrant search: %w", err)
	}
	
	// Маппинг результатов
	results := make([]SearchResult, 0, len(hits))
	for _, hit := range hits {
		result := SearchResult{
			Score: hit.Score,
		}
		
		// Извлечение полей из payload
		if text, ok := hit.Payload["text"].(string); ok {
			result.Text = text
		}
		if source, ok := hit.Payload["source_file"].(string); ok {
			result.Source = source
		}
		if peripheral, ok := hit.Payload["peripheral"].(string); ok {
			result.Peripheral = peripheral
		}
		if chapter, ok := hit.Payload["chapter"].(float64); ok {
			result.Chapter = int(chapter)
		}
		if section, ok := hit.Payload["section"].(string); ok {
			result.Section = section
		}
		if title, ok := hit.Payload["title"].(string); ok {
			result.Title = title
		}
		
		results = append(results, result)
	}
	
	return results, nil
}
