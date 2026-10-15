package tools

import (
	"context"
	"encoding/json"
	"fmt"
)

type Tool interface {
	Name() string
	Description() string
	Execute(ctx context.Context, args json.RawMessage) (interface{}, error)
}

type Registry struct {
	tools map[string]Tool
}

func NewRegistry() *Registry {
	return &Registry{
		tools: make(map[string]Tool),
	}
}

func (r *Registry) Register(tool Tool) {
	r.tools[tool.Name()] = tool
}

func (r *Registry) Get(name string) (Tool, bool) {
	tool, ok := r.tools[name]
	return tool, ok
}

func (r *Registry) List() []map[string]string {
	result := make([]map[string]string, 0, len(r.tools))
	for _, tool := range r.tools {
		result = append(result, map[string]string{
			"name":        tool.Name(),
			"description": tool.Description(),
		})
	}
	return result
}

func (r *Registry) Execute(ctx context.Context, name string, args json.RawMessage) (interface{}, error) {
	tool, ok := r.tools[name]
	if !ok {
		return nil, fmt.Errorf("tool not found: %s", name)
	}
	return tool.Execute(ctx, args)
}
