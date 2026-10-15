package transport

import (
	"bufio"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"os"

	"github.com/mdari/halo-f407-mcp/mcp-server/internal/tools"
)

type StdioTransport struct {
	registry *tools.Registry
	reader   *bufio.Reader
	writer   *bufio.Writer
}

type JSONRPCRequest struct {
	JSONRPC string          `json:"jsonrpc"`
	ID      interface{}     `json:"id"`
	Method  string          `json:"method"`
	Params  json.RawMessage `json:"params,omitempty"`
}

type JSONRPCResponse struct {
	JSONRPC string      `json:"jsonrpc"`
	ID      interface{} `json:"id"`
	Result  interface{} `json:"result,omitempty"`
	Error   *RPCError   `json:"error,omitempty"`
}

type RPCError struct {
	Code    int    `json:"code"`
	Message string `json:"message"`
}

type ToolCallParams struct {
	Name      string          `json:"name"`
	Arguments json.RawMessage `json:"arguments"`
}

func NewStdioTransport(registry *tools.Registry) *StdioTransport {
	return &StdioTransport{
		registry: registry,
		reader:   bufio.NewReader(os.Stdin),
		writer:   bufio.NewWriter(os.Stdout),
	}
}

func (t *StdioTransport) Run() error {
	ctx := context.Background()
	
	for {
		// Чтение строки из stdin
		line, err := t.reader.ReadString('\n')
		if err != nil {
			if err == io.EOF {
				return nil
			}
			return fmt.Errorf("read error: %w", err)
		}
		
		// Парсинг JSON-RPC запроса
		var req JSONRPCRequest
		if err := json.Unmarshal([]byte(line), &req); err != nil {
			t.sendError(nil, -32700, "Parse error")
			continue
		}
		
		// Обработка метода
		switch req.Method {
		case "initialize":
			t.sendResult(req.ID, map[string]interface{}{
				"protocolVersion": "2024-11-05",
				"capabilities": map[string]interface{}{
					"tools": map[string]interface{}{},
				},
				"serverInfo": map[string]interface{}{
					"name":    "stm32f407-mcp",
					"version": "0.1.0",
				},
			})
		
		case "tools/list":
			toolsList := t.registry.List()
			t.sendResult(req.ID, map[string]interface{}{
				"tools": toolsList,
			})
		
		case "tools/call":
			var params ToolCallParams
			if err := json.Unmarshal(req.Params, &params); err != nil {
				t.sendError(req.ID, -32602, "Invalid params")
				continue
			}
			
			result, err := t.registry.Execute(ctx, params.Name, params.Arguments)
			if err != nil {
				t.sendError(req.ID, -32000, err.Error())
				continue
			}
			
			t.sendResult(req.ID, map[string]interface{}{
				"content": []map[string]interface{}{
					{
						"type": "text",
						"text": formatResult(result),
					},
				},
			})
		
		default:
			t.sendError(req.ID, -32601, fmt.Sprintf("Method not found: %s", req.Method))
		}
	}
}

func (t *StdioTransport) sendResult(id interface{}, result interface{}) {
	resp := JSONRPCResponse{
		JSONRPC: "2.0",
		ID:      id,
		Result:  result,
	}
	t.sendResponse(resp)
}

func (t *StdioTransport) sendError(id interface{}, code int, message string) {
	resp := JSONRPCResponse{
		JSONRPC: "2.0",
		ID:      id,
		Error: &RPCError{
			Code:    code,
			Message: message,
		},
	}
	t.sendResponse(resp)
}

func (t *StdioTransport) sendResponse(resp JSONRPCResponse) {
	data, _ := json.Marshal(resp)
	t.writer.Write(data)
	t.writer.WriteString("\n")
	t.writer.Flush()
}

func formatResult(result interface{}) string {
	data, err := json.MarshalIndent(result, "", "  ")
	if err != nil {
		return fmt.Sprintf("%v", result)
	}
	return string(data)
}
