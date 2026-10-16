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
// "toolkit <semver> (contract 1.0)" — semver из Cargo.toml toolkit.
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

// RagTimeout — таймаут RAG-операций (эмбеддинг + Qdrant search), дольше DefaultTimeout.
const RagTimeout = 30 * time.Second

// DocHit — хит поиска по документации (CONTRACT.md §6).
type DocHit struct {
	Text       string  `json:"text"`
	Source     string  `json:"source"`
	Peripheral string  `json:"peripheral"`
	Chapter    int     `json:"chapter"`
	Section    string  `json:"section"`
	Title      string  `json:"title"`
	Score      float64 `json:"score"`
}

// RagQueryResponse — ответ rag-query (CONTRACT.md §6).
type RagQueryResponse struct {
	SchemaVersion int      `json:"schema_version"`
	Command       string   `json:"command"`
	Query         string   `json:"query"`
	Peripheral    string   `json:"peripheral"`
	Results       []DocHit `json:"results"`
}

// RagQueryOptions — опции rag-query.
type RagQueryOptions struct {
	Peripheral string
	Limit      int
}

// RagQuery вызывает `toolkit rag-query` — поиск по документации без AST.
func (c *Client) RagQuery(ctx context.Context, query string, opts RagQueryOptions) (*RagQueryResponse, error) {
	var args []string
	args = append(args, "--query", query)
	if opts.Peripheral != "" {
		args = append(args, "--peripheral", opts.Peripheral)
	}
	args = append(args, "--limit", fmt.Sprint(max(opts.Limit, 1)))

	var resp RagQueryResponse
	if err := c.callArgsTimeout(ctx, "rag-query", "", &resp, args, RagTimeout); err != nil {
		return nil, err
	}
	return &resp, nil
}

// CtxModification — запись об изменении кода (CONTRACT.md §10).
type CtxModification struct {
	File      string `json:"file"`
	Block     string `json:"block"`
	Mode      string `json:"mode"`
	Bytes     int    `json:"bytes"`
	Timestamp int64  `json:"timestamp"`
}

// CtxEntryInput — запись ctx-save (CONTRACT.md §10).
type CtxEntryInput struct {
	Key             string            `json:"key"`
	DocType         string            `json:"doc_type,omitempty"`
	Text            string            `json:"text"`
	Peripherals     []string          `json:"peripherals,omitempty"`
	Interrupts      []string          `json:"interrupts,omitempty"`
	UserPreferences map[string]string `json:"user_preferences,omitempty"`
	Modification    *CtxModification  `json:"modification,omitempty"`
}

// CtxSaveInput — вход ctx-save (stdin JSON, CONTRACT.md §10).
type CtxSaveInput struct {
	ProjectPath string          `json:"project_path"`
	Entries     []CtxEntryInput `json:"entries"`
}

// CtxSaveResponse — ответ ctx-save (CONTRACT.md §10).
type CtxSaveResponse struct {
	SchemaVersion int    `json:"schema_version"`
	Command       string `json:"command"`
	Saved         int    `json:"saved"`
	Collection    string `json:"collection"`
}

// CtxSave вызывает `toolkit ctx-save` — сохранение записей контекста проекта.
func (c *Client) CtxSave(ctx context.Context, in CtxSaveInput) (*CtxSaveResponse, error) {
	payload, err := json.Marshal(in)
	if err != nil {
		return nil, fmt.Errorf("ctx-save payload marshal: %w", err)
	}
	var resp CtxSaveResponse
	if err := c.callArgsTimeout(ctx, "ctx-save", string(payload), &resp, nil, RagTimeout); err != nil {
		return nil, err
	}
	return &resp, nil
}

// CtxEntry — запись контекста в ответе ctx-load (CONTRACT.md §10).
type CtxEntry struct {
	CtxKey          string            `json:"ctx_key"`
	DocType         string            `json:"doc_type"`
	Text            string            `json:"text"`
	Peripherals     []string          `json:"peripherals"`
	Interrupts      []string          `json:"interrupts"`
	UserPreferences map[string]string `json:"user_preferences"`
	Modification    *CtxModification  `json:"modification"`
	Timestamp       *int64            `json:"timestamp"`
}

// CtxLoadResponse — ответ ctx-load (CONTRACT.md §10).
type CtxLoadResponse struct {
	SchemaVersion int        `json:"schema_version"`
	Command       string     `json:"command"`
	ProjectPath   string     `json:"project_path"`
	Count         int        `json:"count"`
	Entries       []CtxEntry `json:"entries"`
}

// CtxLoad вызывает `toolkit ctx-load` — чтение контекста проекта
// (весь проект или одна запись по key).
func (c *Client) CtxLoad(ctx context.Context, project, key string) (*CtxLoadResponse, error) {
	var args []string
	args = append(args, "--project", project)
	if key != "" {
		args = append(args, "--key", key)
	}
	var resp CtxLoadResponse
	if err := c.callArgsTimeout(ctx, "ctx-load", "", &resp, args, RagTimeout); err != nil {
		return nil, err
	}
	// toolkit опускает пустые поля (skip_serializing_if) → JSON null;
	// нормализуем, чтобы MCP output-схема (type: object/array) валидировалась.
	for i := range resp.Entries {
		e := &resp.Entries[i]
		if e.UserPreferences == nil {
			e.UserPreferences = map[string]string{}
		}
		if e.Peripherals == nil {
			e.Peripherals = []string{}
		}
		if e.Interrupts == nil {
			e.Interrupts = []string{}
		}
	}
	return &resp, nil
}

// CtxHit — хит ctx-search (CONTRACT.md §10).
type CtxHit struct {
	Score       float64 `json:"score"`
	ProjectPath string  `json:"project_path"`
	CtxKey      string  `json:"ctx_key"`
	DocType     string  `json:"doc_type"`
	Text        string  `json:"text"`
}

// CtxSearchResponse — ответ ctx-search (CONTRACT.md §10).
type CtxSearchResponse struct {
	SchemaVersion int      `json:"schema_version"`
	Command       string   `json:"command"`
	ProjectPath   string   `json:"project_path"`
	Query         string   `json:"query"`
	Results       []CtxHit `json:"results"`
}

// CtxSearch вызывает `toolkit ctx-search` — семантический поиск
// по контексту проекта.
func (c *Client) CtxSearch(ctx context.Context, project, query string, limit int) (*CtxSearchResponse, error) {
	args := []string{"--project", project, "--query", query, "--limit", fmt.Sprint(max(limit, 1))}
	var resp CtxSearchResponse
	if err := c.callArgsTimeout(ctx, "ctx-search", "", &resp, args, RagTimeout); err != nil {
		return nil, err
	}
	if resp.Results == nil {
		resp.Results = []CtxHit{}
	}
	return &resp, nil
}

// call — общий вызов: code в stdin, JSON-ответ из stdout, JSON-ошибка из stderr.
func (c *Client) call(ctx context.Context, command, code string, out any) error {
	return c.callArgs(ctx, command, code, out, nil)
}

func (c *Client) callArgs(ctx context.Context, command, code string, out any, extraArgs []string) error {
	return c.callArgsTimeout(ctx, command, code, out, extraArgs, c.timeout)
}

func (c *Client) callArgsTimeout(ctx context.Context, command, code string, out any, extraArgs []string, timeout time.Duration) error {
	ctx, cancel := context.WithTimeout(ctx, timeout)
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
			return fmt.Errorf("toolkit %s timed out after %s", command, timeout)
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
