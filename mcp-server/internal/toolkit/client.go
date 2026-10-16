// Package toolkit — Go-клиент Rust CLI toolkit (CONTRACT.md, версия 1.0).
//
// Транспорт: JSON по stdin/stdout exec.Command, ошибки — JSON в stderr
// вида {"error": {"code", "message", "command"}}. Путь бинарника —
// env TOOLKIT_PATH (не хардкодится: darwin/linux/windows различаются).
package toolkit

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"strings"
	"time"
)

// ContractVersion — ожидаемая мажорная версия контракта toolkit.
const ContractVersion = "1.0"

// DefaultTimeout — таймаут на один вызов toolkit.
const DefaultTimeout = 5 * time.Second

// SchemaVersion — версия схем, отправляемая в запросах.
const SchemaVersion = 1

// Client вызывает бинарник toolkit по путю из TOOLKIT_PATH.
type Client struct {
	path    string
	timeout time.Duration
}

// Position — позиция в исходном коде (CONTRACT.md §1).
type Position struct {
	ByteStart int `json:"byte_start"`
	ByteEnd   int `json:"byte_end"`
	Line      int `json:"line"`
	Col       int `json:"col"`
}

// Violation — нарушение lint-правила (CONTRACT.md §3).
type Violation struct {
	Rule     string   `json:"rule"`
	Severity string   `json:"severity"`
	Message  string   `json:"message"`
	Position Position `json:"position"`
	Hint     string   `json:"hint"`
}

// Summary — сводка по violations (CONTRACT.md §3).
type Summary struct {
	Errors   int `json:"errors"`
	Warnings int `json:"warnings"`
	Infos    int `json:"infos"`
}

// LintResponse — ответ lint/flow-analysis (CONTRACT.md §3).
type LintResponse struct {
	SchemaVersion int              `json:"schema_version"`
	Command       string           `json:"command"`
	Summary       Summary          `json:"summary"`
	Violations    []Violation      `json:"violations"`
	Flows         *json.RawMessage `json:"flows,omitempty"`
}

// Insertion — вставка doc-комментария (CONTRACT.md §4).
type Insertion struct {
	Position Position `json:"position"`
	Text     string   `json:"text"`
}

// DocsResponse — ответ generate-docs (CONTRACT.md §4).
type DocsResponse struct {
	SchemaVersion       int         `json:"schema_version"`
	Command             string      `json:"command"`
	Insertions          []Insertion `json:"insertions"`
	FunctionsDocumented int         `json:"functions_documented"`
	FunctionsSkipped    int         `json:"functions_skipped"`
}

// FunctionItem — извлечённая функция (CONTRACT.md §2).
type FunctionItem struct {
	Name       string      `json:"name"`
	Signature  string      `json:"signature"`
	Position   Position    `json:"position"`
	Attributes []Attribute `json:"attributes"`
}

// Attribute — атрибут объявления (CONTRACT.md §2).
type Attribute struct {
	Name string   `json:"name"`
	Args []string `json:"args"`
}

// VariableItem — извлечённая переменная (CONTRACT.md §2).
type VariableItem struct {
	Name       string      `json:"name"`
	Type       string      `json:"type"`
	Storage    string      `json:"storage"`
	Position   Position    `json:"position"`
	Attributes []Attribute `json:"attributes"`
}

// CallItem — извлечённый вызов функции (CONTRACT.md §2).
type CallItem struct {
	Callee   string   `json:"callee"`
	Position Position `json:"position"`
}

// SourceStats — размер исходника (CONTRACT.md §2).
type SourceStats struct {
	Bytes int `json:"bytes"`
	Lines int `json:"lines"`
}

// AnalyzeResponse — ответ analyze (CONTRACT.md §2).
type AnalyzeResponse struct {
	SchemaVersion int               `json:"schema_version"`
	Command       string            `json:"command"`
	SourceStats   SourceStats       `json:"source_stats"`
	Functions     []FunctionItem    `json:"functions"`
	Variables     []VariableItem    `json:"variables"`
	HalCalls      []CallItem        `json:"hal_calls"`
	ParseErrors   []json.RawMessage `json:"parse_errors"`
}

// toolkitError — формат ошибки из stderr (CONTRACT.md §8).
type toolkitError struct {
	Error struct {
		Code    string `json:"code"`
		Message string `json:"message"`
		Command string `json:"command"`
	} `json:"error"`
}

// VersionInfo — результат парсинга `toolkit --version`.
type VersionInfo struct {
	Raw        string
	MajorMinor string
}

// NewClient читает TOOLKIT_PATH. Пустой путь — ошибка (осмысленное
// сообщение вместо запуска с пустой командой).
func NewClient() (*Client, error) {
	path := os.Getenv("TOOLKIT_PATH")
	if path == "" {
		return nil, fmt.Errorf("TOOLKIT_PATH is not set: point it to the toolkit binary (e.g. TOOLKIT_PATH=/usr/local/bin/toolkit)")
	}
	return &Client{path: path, timeout: DefaultTimeout}, nil
}

// NewClientWithPath — клиент с явным путём (тесты, альтернативные размещения).
func NewClientWithPath(path string, timeout time.Duration) *Client {
	if timeout <= 0 {
		timeout = DefaultTimeout
	}
	return &Client{path: path, timeout: timeout}
}

// Path возвращает путь к бинарнику.
func (c *Client) Path() string { return c.path }

// Version выполняет `toolkit --version` и парсит вывод
// "toolkit 0.1.0 (contract 1.0)".
func (c *Client) Version(ctx context.Context) (VersionInfo, error) {
	ctx, cancel := context.WithTimeout(ctx, c.timeout)
	defer cancel()
	out, err := exec.CommandContext(ctx, c.path, "--version").Output()
	if err != nil {
		return VersionInfo{}, fmt.Errorf("toolkit --version failed: %w", err)
	}
	raw := strings.TrimSpace(string(out))
	parts := strings.Fields(raw)
	if len(parts) < 2 {
		return VersionInfo{}, fmt.Errorf("cannot parse toolkit version output: %q", raw)
	}
	version := parts[1]

	contract := ""
	for i, p := range parts {
		if p == "(contract" && i+1 < len(parts) {
			contract = strings.TrimSuffix(parts[i+1], ")")
		}
	}
	if contract != "" {
		if strings.SplitN(contract, ".", 2)[0] != strings.SplitN(ContractVersion, ".", 2)[0] {
			return VersionInfo{}, fmt.Errorf("toolkit contract %s incompatible with expected %s", contract, ContractVersion)
		}
	}
	return VersionInfo{Raw: raw, MajorMinor: version}, nil
}

// Analyze вызывает `toolkit analyze`.
func (c *Client) Analyze(ctx context.Context, code string) (*AnalyzeResponse, error) {
	var resp AnalyzeResponse
	if err := c.call(ctx, "analyze", code, &resp); err != nil {
		return nil, err
	}
	return &resp, nil
}

// LintOptions — опции lint.
type LintOptions struct {
	Flow bool
}

// Lint вызывает `toolkit lint` (или flow-analysis при opts.Flow).
func (c *Client) Lint(ctx context.Context, code string, opts LintOptions) (*LintResponse, error) {
	command := "lint"
	if opts.Flow {
		command = "flow-analysis"
	}
	var resp LintResponse
	if err := c.call(ctx, command, code, &resp); err != nil {
		return nil, err
	}
	return &resp, nil
}

// GenerateDocs вызывает `toolkit generate-docs`.
func (c *Client) GenerateDocs(ctx context.Context, code string) (*DocsResponse, error) {
	var resp DocsResponse
	if err := c.call(ctx, "generate-docs", code, &resp); err != nil {
		return nil, err
	}
	return &resp, nil
}

// RagSearch вызывает `toolkit rag-search`.
type RagSearchOptions struct {
	RuleContext string
	Limit       int
}

// RagResponse — ответ rag-search (CONTRACT.md §5).
type RagResponse struct {
	SchemaVersion int               `json:"schema_version"`
	Command       string            `json:"command"`
	Violations    []Violation       `json:"violations"`
	RagRefs       []json.RawMessage `json:"rag_refs"`
	RagDegraded   bool              `json:"rag_degraded"`
}

// RagSearch вызывает `toolkit rag-search` с правилом контекстом и лимитом.
func (c *Client) RagSearch(ctx context.Context, code string, opts RagSearchOptions) (*RagResponse, error) {
	var resp RagResponse
	if err := c.callArgs(ctx, "rag-search", code, &resp, ragArgs(opts)); err != nil {
		return nil, err
	}
	return &resp, nil
}

func ragArgs(opts RagSearchOptions) []string {
	var args []string
	if opts.RuleContext != "" {
		args = append(args, "--rule-context", opts.RuleContext)
	}
	if opts.Limit > 0 {
		args = append(args, "--limit", fmt.Sprint(opts.Limit))
	}
	return args
}

// call — общий вызов: code в stdin, JSON-ответ из stdout, JSON-ошибка из stderr.
func (c *Client) call(ctx context.Context, command, code string, out any) error {
	return c.callArgs(ctx, command, code, out, nil)
}

func (c *Client) callArgs(ctx context.Context, command, code string, out any, extraArgs []string) error {
	ctx, cancel := context.WithTimeout(ctx, c.timeout)
	defer cancel()

	args := append([]string{command}, extraArgs...)
	cmd := exec.CommandContext(ctx, c.path, args...)
	cmd.Stdin = strings.NewReader(code)

	var stdout, stderr strings.Builder
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr

	err := cmd.Run()
	if err != nil {
		if ctx.Err() == context.DeadlineExceeded {
			return fmt.Errorf("toolkit %s timed out after %s", command, c.timeout)
		}
		return humanError(command, err, stderr.String())
	}

	if err := json.Unmarshal([]byte(stdout.String()), out); err != nil {
		return fmt.Errorf("toolkit %s returned invalid JSON: %w; stdout: %q", command, err, truncate(stdout.String(), 200))
	}
	return nil
}

// humanError превращает non-zero exit в человекочитаемую ошибку,
// по возможности извлекая JSON-ошибку CONTRACT.md из stderr.
func humanError(command string, runErr error, stderr string) error {
	var te toolkitError
	if json.Unmarshal([]byte(strings.TrimSpace(stderr)), &te) == nil && te.Error.Code != "" {
		if te.Error.Command == "" {
			te.Error.Command = command
		}
		return fmt.Errorf("toolkit %s: %s: %s", te.Error.Command, te.Error.Code, te.Error.Message)
	}
	var exitErr *exec.ExitError
	msg := runErr.Error()
	if e := stderr; strings.TrimSpace(e) != "" {
		msg = fmt.Sprintf("%s: %s", msg, truncate(strings.TrimSpace(e), 200))
	}
	if e, ok := runErr.(*exec.ExitError); ok {
		exitErr = e
		_ = exitErr
	}
	return fmt.Errorf("toolkit %s failed: %s", command, msg)
}

func truncate(s string, n int) string {
	if len(s) <= n {
		return s
	}
	return s[:n] + "..."
}
