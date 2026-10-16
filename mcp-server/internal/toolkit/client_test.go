package toolkit

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"runtime"
	"strings"
	"testing"
	"time"
)

// stubScript возвращает shell-скрипт, который печатает заданный stdout/stderr
// и выходит с заданным кодом; работает и как bash, и как .bat через cmd shim.
func stubScript(t *testing.T, stdout, stderr string, exit int) string {
	t.Helper()
	dir := t.TempDir()
	if runtime.GOOS == "windows" {
		path := filepath.Join(dir, "stub.bat")
		script := "@echo off\r\n" +
			"echo " + strings.ReplaceAll(stdout, "\"", "") + "\r\n" +
			"if not \"" + strings.ReplaceAll(stderr, "\"", "") + "\"==\"\" echo " + strings.ReplaceAll(stderr, "\"", "") + " 1>&2\r\n" +
			"exit /b " + itoa(exit) + "\r\n"
		if err := os.WriteFile(path, []byte(script), 0o755); err != nil {
			t.Fatal(err)
		}
		return path
	}
	path := filepath.Join(dir, "stub.sh")
	var script strings.Builder
	script.WriteString("#!/bin/sh\n")
	if stdout != "" {
		script.WriteString("printf '%s\\n' '" + stdout + "'\n")
	}
	if stderr != "" {
		script.WriteString("printf '%s\\n' '" + stderr + "' 1>&2\n")
	}
	script.WriteString("exit " + itoa(exit) + "\n")
	if err := os.WriteFile(path, []byte(script.String()), 0o755); err != nil {
		t.Fatal(err)
	}
	return path
}

// stubCat — бинарник, который не читает stdin, но валидирует вход по коду.
// Для положительных сценариев делаем заглушку, печатающую фиксированный ответ.
func itoa(n int) string {
	if n == 0 {
		return "0"
	}
	digits := ""
	for n > 0 {
		digits = string(rune('0'+n%10)) + digits
		n /= 10
	}
	return digits
}

func analyzeStubResponse() string {
	return `{"schema_version":1,"command":"analyze","source_stats":{"bytes":30,"lines":1},"parse_errors":[],"functions":[{"name":"main","position":{"byte_start":0,"byte_end":30,"line":1,"col":1}}],"variables":[],"hal_calls":[]}`
}

func TestAnalyzeParsesResponse(t *testing.T) {
	stub := stubScript(t, analyzeStubResponse(), "", 0)
	c := NewClientWithPath(stub, 5*time.Second)

	resp, err := c.Analyze(context.Background(), "int main(void){return 0;}")
	if err != nil {
		t.Fatalf("Analyze: %v", err)
	}
	if resp.SchemaVersion != 1 || resp.Command != "analyze" {
		t.Fatalf("unexpected header: %+v", resp)
	}
	if len(resp.Functions) != 1 || resp.Functions[0].Name != "main" {
		t.Fatalf("unexpected functions: %+v", resp.Functions)
	}
	if resp.Functions[0].Position.Line != 1 {
		t.Fatalf("unexpected position: %+v", resp.Functions[0].Position)
	}
}

func TestLintParsesResponse(t *testing.T) {
	stub := stubScript(t, `{"schema_version":1,"command":"lint","summary":{"errors":1,"warnings":0,"infos":0},"violations":[{"rule":"dma_buffer_in_ccmram","severity":"error","message":"buf in .ccmram without DMA link","position":{"byte_start":0,"byte_end":10,"line":1,"col":1},"hint":"link DMA"}],"flows":null}`, "", 0)
	c := NewClientWithPath(stub, 5*time.Second)

	resp, err := c.Lint(context.Background(), "code", LintOptions{})
	if err != nil {
		t.Fatalf("Lint: %v", err)
	}
	if len(resp.Violations) != 1 || resp.Violations[0].Rule != "dma_buffer_in_ccmram" {
		t.Fatalf("unexpected violations: %+v", resp.Violations)
	}
	if resp.Summary.Errors != 1 {
		t.Fatalf("unexpected summary: %+v", resp.Summary)
	}
}

func TestGenerateDocsParsesResponse(t *testing.T) {
	stub := stubScript(t, `{"schema_version":1,"command":"generate-docs","insertions":[{"position":{"byte_start":0,"byte_end":0,"line":1,"col":1},"text":"/**\n * @brief f\n */\n"}],"functions_documented":1,"functions_skipped":0}`, "", 0)
	c := NewClientWithPath(stub, 5*time.Second)

	resp, err := c.GenerateDocs(context.Background(), "int f(void);")
	if err != nil {
		t.Fatalf("GenerateDocs: %v", err)
	}
	if len(resp.Insertions) != 1 || resp.Insertions[0].Position.Line != 1 {
		t.Fatalf("unexpected insertions: %+v", resp.Insertions)
	}
}

func TestRagSearchArgsAndParsing(t *testing.T) {
	// Заглушка печатает ответ только если аргументы совпали с ожидаемыми.
	dir := t.TempDir()
	path := filepath.Join(dir, "ragstub.sh")
	script := `#!/bin/sh
if [ "$1" = "rag-search" ] && [ "$2" = "--rule-context" ] && [ "$3" = "DMA buffer" ] && [ "$4" = "--limit" ] && [ "$5" = "7" ]; then
  printf '%s\n' '{"schema_version":1,"command":"rag-search","violations":[],"rag_refs":[{"score":0.9}],"rag_degraded":false}'
else
  printf '%s\n' '{"error":{"code":"E_INTERNAL","message":"bad args","command":"rag-search"}}' 1>&2
  exit 2
fi
`
	if err := os.WriteFile(path, []byte(script), 0o755); err != nil {
		t.Fatal(err)
	}
	c := NewClientWithPath(path, 5*time.Second)

	resp, err := c.RagSearch(context.Background(), "code", RagSearchOptions{RuleContext: "DMA buffer", Limit: 7})
	if err != nil {
		t.Fatalf("RagSearch: %v", err)
	}
	if resp.RagDegraded {
		t.Fatal("expected rag_degraded=false")
	}
	if len(resp.RagRefs) != 1 {
		t.Fatalf("unexpected rag_refs: %s", resp.RagRefs)
	}
}

func TestNonZeroExitWithJSONError(t *testing.T) {
	stub := stubScript(t, "", `{"error":{"code":"E_INPUT_READ","message":"failed to read stdin","command":"analyze"}}`, 2)
	c := NewClientWithPath(stub, 5*time.Second)

	_, err := c.Analyze(context.Background(), "code")
	if err == nil {
		t.Fatal("expected error")
	}
	msg := err.Error()
	if !strings.Contains(msg, "E_INPUT_READ") || !strings.Contains(msg, "failed to read stdin") {
		t.Fatalf("expected contract error, got: %s", msg)
	}
	if strings.Contains(msg, "exit status") {
		t.Fatalf("expected human-readable error, got: %s", msg)
	}
}

func TestNonZeroExitWithoutJSONError(t *testing.T) {
	stub := stubScript(t, "", "panic: something exploded", 3)
	c := NewClientWithPath(stub, 5*time.Second)

	_, err := c.Lint(context.Background(), "code", LintOptions{Flow: true})
	if err == nil {
		t.Fatal("expected error")
	}
	if !strings.Contains(err.Error(), "panic: something exploded") {
		t.Fatalf("expected stderr content in error, got: %s", err)
	}
}

func TestInvalidJSONResponse(t *testing.T) {
	stub := stubScript(t, "not json at all", "", 0)
	c := NewClientWithPath(stub, 5*time.Second)

	_, err := c.Analyze(context.Background(), "code")
	if err == nil || !strings.Contains(err.Error(), "invalid JSON") {
		t.Fatalf("expected invalid JSON error, got: %v", err)
	}
}

func TestMissingBinary(t *testing.T) {
	c := NewClientWithPath(filepath.Join(t.TempDir(), "no-such-toolkit"), 5*time.Second)
	_, err := c.Analyze(context.Background(), "code")
	if err == nil {
		t.Fatal("expected error for missing binary")
	}
	if !strings.Contains(err.Error(), "no-such-toolkit") {
		t.Fatalf("expected path in error, got: %s", err)
	}
}

func TestTimeout(t *testing.T) {
	if runtime.GOOS == "windows" {
		t.Skip("sleep script not portable")
	}
	dir := t.TempDir()
	path := filepath.Join(dir, "slow.sh")
	if err := os.WriteFile(path, []byte("#!/bin/sh\nsleep 5\n"), 0o755); err != nil {
		t.Fatal(err)
	}
	c := NewClientWithPath(path, 100*time.Millisecond)

	start := time.Now()
	_, err := c.Analyze(context.Background(), "code")
	if err == nil || !strings.Contains(err.Error(), "timed out") {
		t.Fatalf("expected timeout error, got: %v", err)
	}
	if time.Since(start) > 2*time.Second {
		t.Fatalf("timeout not enforced promptly")
	}
}

func TestNewClientRequiresEnv(t *testing.T) {
	t.Setenv("TOOLKIT_PATH", "")
	if _, err := NewClient(); err == nil || !strings.Contains(err.Error(), "TOOLKIT_PATH is not set") {
		t.Fatalf("expected TOOLKIT_PATH error, got: %v", err)
	}
	t.Setenv("TOOLKIT_PATH", "/opt/toolkit")
	c, err := NewClient()
	if err != nil {
		t.Fatalf("NewClient: %v", err)
	}
	if c.Path() != "/opt/toolkit" {
		t.Fatalf("unexpected path: %s", c.Path())
	}
}

func TestVersionParsesContract(t *testing.T) {
	stub := stubScript(t, "toolkit 0.1.0 (contract 1.0)", "", 0)
	c := NewClientWithPath(stub, 5*time.Second)

	v, err := c.Version(context.Background())
	if err != nil {
		t.Fatalf("Version: %v", err)
	}
	if v.MajorMinor != "0.1.0" {
		t.Fatalf("unexpected MajorMinor: %q", v.MajorMinor)
	}
}

func TestVersionIncompatibleMajor(t *testing.T) {
	stub := stubScript(t, "toolkit 9.9.9 (contract 9.0)", "", 0)
	c := NewClientWithPath(stub, 5*time.Second)

	_, err := c.Version(context.Background())
	if err == nil || !strings.Contains(err.Error(), "incompatible") {
		t.Fatalf("expected incompatibility error, got: %v", err)
	}
}

func TestRealToolkitBinary(t *testing.T) {
	bin := os.Getenv("TOOLKIT_PATH")
	if bin == "" {
		t.Skip("TOOLKIT_PATH not set; build toolkit with cargo build")
	}
	if _, err := os.Stat(bin); err != nil {
		t.Skipf("TOOLKIT_PATH %s not found: %v", bin, err)
	}
	c := NewClientWithPath(bin, 5*time.Second)

	resp, err := c.Lint(context.Background(), "__attribute__((section(\".ccmram\"))) static uint8_t b[16];\nvoid start(void){ HAL_DMA_Start(&h, b, b, 16); }\n", LintOptions{})
	if err != nil {
		t.Fatalf("Lint: %v", err)
	}
	if resp.SchemaVersion != 1 {
		t.Fatalf("unexpected schema_version: %d", resp.SchemaVersion)
	}
	if len(resp.Violations) == 0 || resp.Violations[0].Rule != "dma_buffer_in_ccmram" {
		b, _ := json.Marshal(resp)
		t.Fatalf("expected dma_buffer_in_ccmram violation, got: %s", b)
	}
	if resp.Summary.Errors != 1 {
		t.Fatalf("unexpected summary: %+v", resp.Summary)
	}
	b, _ := json.Marshal(resp.Summary)
	t.Logf("real toolkit summary: %s", b)
}
