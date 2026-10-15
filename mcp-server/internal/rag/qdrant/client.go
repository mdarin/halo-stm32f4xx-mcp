package qdrant

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"time"
)

type Client struct {
	baseURL    string
	httpClient *http.Client
}

type SearchRequest struct {
	Collection string                 `json:"collection"`
	Vector     []float32              `json:"vector"`
	Limit      int                    `json:"limit"`
	Filter     map[string]interface{} `json:"filter,omitempty"`
}

type SearchResponse struct {
	Result []SearchHit `json:"result"`
}

type SearchHit struct {
	ID      int64                  `json:"id"`
	Score   float32                `json:"score"`
	Payload map[string]interface{} `json:"payload"`
}

func NewClient(baseURL string) *Client {
	return &Client{
		baseURL: baseURL,
		httpClient: &http.Client{
			Timeout: 30 * time.Second,
		},
	}
}

func (c *Client) Search(ctx context.Context, collection string, vector []float32, limit int, filter map[string]interface{}) ([]SearchHit, error) {
	reqBody := map[string]interface{}{
		"vector": vector,
		"limit":  limit,
	}
	
	if filter != nil {
		reqBody["filter"] = filter
	}
	
	body, err := json.Marshal(reqBody)
	if err != nil {
		return nil, fmt.Errorf("marshal request: %w", err)
	}
	
	url := fmt.Sprintf("%s/collections/%s/points/search", c.baseURL, collection)
	req, err := http.NewRequestWithContext(ctx, "POST", url, bytes.NewReader(body))
	if err != nil {
		return nil, fmt.Errorf("create request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")
	
	resp, err := c.httpClient.Do(req)
	if err != nil {
		return nil, fmt.Errorf("do request: %w", err)
	}
	defer resp.Body.Close()
	
	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("qdrant returned status %d", resp.StatusCode)
	}
	
	var searchResp SearchResponse
	if err := json.NewDecoder(resp.Body).Decode(&searchResp); err != nil {
		return nil, fmt.Errorf("decode response: %w", err)
	}
	
	return searchResp.Result, nil
}
