package main

import (
	"context"
	"encoding/json"
	"fmt"
	"os"

	"github.com/mdari/halo-f407-mcp/mcp-server/internal/toolkit"
	"github.com/modelcontextprotocol/go-sdk/mcp"
)

// AstAnalyzeInput — входные параметры для ast_analyze_code
type AstAnalyzeInput struct {
	Code     string `json:"code,omitempty" jsonschema:"C source code to analyze (alternative to file_path)"`
	FilePath string `json:"file_path,omitempty" jsonschema:"path to a .c file to analyze (alternative to code)"`
}

// AstAnalyzeOutput — результат AST-анализа (прозрачный pass-through toolkit analyze)
type AstAnalyzeOutput struct {
	SchemaVersion int                    `json:"schema_version"`
	SourceStats   toolkit.SourceStats    `json:"source_stats"`
	Functions     []toolkit.FunctionItem `json:"functions"`
	Variables     []toolkit.VariableItem `json:"variables"`
	HalCalls      []toolkit.CallItem     `json:"hal_calls"`
	ParseErrors   []json.RawMessage      `json:"parse_errors"`
}

// GenerateDocCommentsInput — входные параметры для generate_doc_comments
type GenerateDocCommentsInput struct {
	Code     string `json:"code,omitempty" jsonschema:"C source code to document (alternative to file_path)"`
	FilePath string `json:"file_path,omitempty" jsonschema:"path to a .c file (alternative to code)"`
}

// GenerateDocCommentsOutput — результат generate-docs (прозрачный pass-through)
type GenerateDocCommentsOutput struct {
	SchemaVersion       int                 `json:"schema_version"`
	Insertions          []toolkit.Insertion `json:"insertions"`
	FunctionsDocumented int                 `json:"functions_documented"`
	FunctionsSkipped    int                 `json:"functions_skipped"`
}

// AdvancedLintInput — входные параметры для advanced_lint
type AdvancedLintInput struct {
	Code     string `json:"code,omitempty" jsonschema:"C source code to lint (alternative to file_path)"`
	FilePath string `json:"file_path,omitempty" jsonschema:"path to a .c file (alternative to code)"`
	Flow     bool   `json:"flow,omitempty" jsonschema:"track variable usage in DMA paths (flow-analysis)"`
}

// AdvancedLintOutput — результат lint/flow-analysis (прозрачный pass-through)
type AdvancedLintOutput struct {
	SchemaVersion int                 `json:"schema_version"`
	Summary       toolkit.Summary     `json:"summary"`
	Violations    []toolkit.Violation `json:"violations"`
	Flows         any                 `json:"flows,omitempty"`
}

// toolkitCodeInput — code/file_path → исходник для toolkit.
type toolkitCodeInput struct {
	Code     string
	FilePath string
}

func toolkitCode(input toolkitCodeInput) (string, error) {
	if input.Code != "" {
		return input.Code, nil
	}
	if input.FilePath != "" {
		data, err := os.ReadFile(input.FilePath)
		if err != nil {
			return "", fmt.Errorf("failed to read file_path: %w", err)
		}
		return string(data), nil
	}
	return "", fmt.Errorf("provide either 'code' or 'file_path'")
}

func requireToolkit() error {
	if globalToolkit == nil {
		return fmt.Errorf("toolkit is not available: the MCP server was started without TOOLKIT_PATH (build the Rust CLI and set TOOLKIT_PATH to the binary)")
	}
	return nil
}

// astAnalyzeHandler — обработчик инструмента ast_analyze_code
func astAnalyzeHandler(ctx context.Context, req *mcp.CallToolRequest, input AstAnalyzeInput) (
	*mcp.CallToolResult,
	AstAnalyzeOutput,
	error,
) {
	if err := requireToolkit(); err != nil {
		return nil, AstAnalyzeOutput{}, err
	}
	code, err := toolkitCode(toolkitCodeInput{Code: input.Code, FilePath: input.FilePath})
	if err != nil {
		return nil, AstAnalyzeOutput{}, err
	}
	resp, err := globalToolkit.Analyze(ctx, code)
	if err != nil {
		return nil, AstAnalyzeOutput{}, err
	}
	return nil, AstAnalyzeOutput{
		SchemaVersion: resp.SchemaVersion,
		SourceStats:   resp.SourceStats,
		Functions:     resp.Functions,
		Variables:     resp.Variables,
		HalCalls:      resp.HalCalls,
		ParseErrors:   resp.ParseErrors,
	}, nil
}

// generateDocCommentsHandler — обработчик инструмента generate_doc_comments
func generateDocCommentsHandler(ctx context.Context, req *mcp.CallToolRequest, input GenerateDocCommentsInput) (
	*mcp.CallToolResult,
	GenerateDocCommentsOutput,
	error,
) {
	if err := requireToolkit(); err != nil {
		return nil, GenerateDocCommentsOutput{}, err
	}
	code, err := toolkitCode(toolkitCodeInput{Code: input.Code, FilePath: input.FilePath})
	if err != nil {
		return nil, GenerateDocCommentsOutput{}, err
	}
	resp, err := globalToolkit.GenerateDocs(ctx, code)
	if err != nil {
		return nil, GenerateDocCommentsOutput{}, err
	}
	return nil, GenerateDocCommentsOutput{
		SchemaVersion:       resp.SchemaVersion,
		Insertions:          resp.Insertions,
		FunctionsDocumented: resp.FunctionsDocumented,
		FunctionsSkipped:    resp.FunctionsSkipped,
	}, nil
}

// advancedLintHandler — обработчик инструмента advanced_lint
func advancedLintHandler(ctx context.Context, req *mcp.CallToolRequest, input AdvancedLintInput) (
	*mcp.CallToolResult,
	AdvancedLintOutput,
	error,
) {
	if err := requireToolkit(); err != nil {
		return nil, AdvancedLintOutput{}, err
	}
	code, err := toolkitCode(toolkitCodeInput{Code: input.Code, FilePath: input.FilePath})
	if err != nil {
		return nil, AdvancedLintOutput{}, err
	}
	resp, err := globalToolkit.Lint(ctx, code, toolkit.LintOptions{Flow: input.Flow})
	if err != nil {
		return nil, AdvancedLintOutput{}, err
	}
	return nil, AdvancedLintOutput{
		SchemaVersion: resp.SchemaVersion,
		Summary:       resp.Summary,
		Violations:    resp.Violations,
		Flows:         resp.Flows,
	}, nil
}
