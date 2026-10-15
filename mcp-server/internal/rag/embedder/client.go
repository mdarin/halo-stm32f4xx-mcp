package embedder

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
	model      string
	httpClient *http.Client
}

type EmbedResponse struct {
	Embeddings [][]float32 `json:"embeddings"`
}

func NewClient(baseURL, model string) *Client {
	return &Client{
		baseURL: baseURL,
		model:   model,
		httpClient: &http.Client{
			Timeout: 60 * time.Second,
		},
	}
}

func (c *Client) Embed(ctx context.Context, text string) ([]float32, error) {
	var lastErr error

	// Retry logic: 3 attempts with exponential backoff
	for attempt := 0; attempt < 3; attempt++ {
		vector, err := c.embed(ctx, text)
		if err == nil {
			return vector, nil
		}
		lastErr = err

		if attempt < 2 {
			time.Sleep(time.Duration(attempt+1) * time.Second)
		}
	}

	return nil, fmt.Errorf("ollama unavailable after 3 attempts: %w", lastErr)
}

func (c *Client) embed(ctx context.Context, text string) ([]float32, error) {
	reqBody := map[string]interface{}{
		"model": c.model,
		"input": text,
	}

	body, err := json.Marshal(reqBody)
	if err != nil {
		return nil, fmt.Errorf("marshal request: %w", err)
	}

	url := fmt.Sprintf("%s/api/embed", c.baseURL)
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
		return nil, fmt.Errorf("ollama returned status %d", resp.StatusCode)
	}

	var embedResp EmbedResponse
	if err := json.NewDecoder(resp.Body).Decode(&embedResp); err != nil {
		return nil, fmt.Errorf("decode response: %w", err)
	}

	if len(embedResp.Embeddings) == 0 {
		return nil, fmt.Errorf("empty embedding returned")
	}

	return embedResp.Embeddings[0], nil
}
