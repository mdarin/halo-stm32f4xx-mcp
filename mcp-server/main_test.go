package main

import (
	"context"
	"encoding/json"
	"os"
	"os/exec"
	"path/filepath"
	"testing"
	"time"

	"github.com/mdari/halo-f407-mcp/mcp-server/internal/cubemx"
	"github.com/mdari/halo-f407-mcp/mcp-server/internal/toolkit"
)

// TestLogModificationE2E — Ф5-4 DoD: logModification пишет modification
// через реальный toolkit ctx-save, ctx-load показывает запись.
// Требует TOOLKIT_PATH (билдится cargo build) и локальный Qdrant/Ollama.
func TestLogModificationE2E(t *testing.T) {
	bin := os.Getenv("TOOLKIT_PATH")
	if bin == "" {
		t.Skip("TOOLKIT_PATH not set")
	}
	tc := toolkit.NewClientWithPath(bin, 30*time.Second)
	globalToolkit = tc
	defer func() { globalToolkit = nil }()

	file := "/Users/mikhail/projects/halo-stm32f4xx-mcp/shift-commander/src/main.c"
	ts := time.Now().Unix()
	result := &cubemx.InsertResult{
		FilePath:     file,
		BlockName:    "e2etest",
		Mode:         "append",
		BytesWritten: 42,
		Success:      true,
	}

	logModification(InsertCodeInput{}, result)

	// logModification — горутина; ждём завершения.
	deadline := time.Now().Add(30 * time.Second)
	for time.Now().Before(deadline) {
		time.Sleep(500 * time.Millisecond)
		cmd := exec.Command(bin, "ctx-load", "--project",
			"/Users/mikhail/projects/halo-stm32f4xx-mcp/shift-commander")
		out, err := cmd.Output()
		if err != nil {
			t.Fatalf("ctx-load: %v", err)
		}
		var resp struct {
			Entries []struct {
				CtxKey       string `json:"ctx_key"`
				DocType      string `json:"doc_type"`
				Modification *struct {
					Block     string `json:"block"`
					Timestamp int64  `json:"timestamp"`
				} `json:"modification"`
			} `json:"entries"`
		}
		if err := json.Unmarshal(out, &resp); err != nil {
			t.Fatalf("ctx-load decode: %v", err)
		}
		for _, e := range resp.Entries {
			if e.CtxKey == "mod:main.c:e2etest:0" {
				return
			}
		}
		// ts может быть любым; ищем по block+recent timestamp
		for _, e := range resp.Entries {
			if e.DocType == "modification" && e.Modification != nil &&
				e.Modification.Block == "e2etest" && e.Modification.Timestamp >= ts-2 {
				return
			}
		}
	}
	t.Fatal("modification entry not found in ctx-load after 30s")
}

// TestContextLogDisabled — env-флаг выключает лог.
func TestContextLogDisabled(t *testing.T) {
	t.Setenv("DISABLE_CONTEXT_LOG", "1")
	if !contextLogDisabled() {
		t.Fatal("expected disabled")
	}
	t.Setenv("DISABLE_CONTEXT_LOG", "")
	if contextLogDisabled() {
		t.Fatal("expected enabled by default")
	}
}

// TestFindProjectRoot — .ioc ищется вверх по дереву.
func TestFindProjectRoot(t *testing.T) {
	got := findProjectRoot("/Users/mikhail/projects/halo-stm32f4xx-mcp/shift-commander/src/main.c")
	want := "/Users/mikhail/projects/halo-stm32f4xx-mcp/shift-commander"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
	if got := findProjectRoot("/tmp"); got != filepath.Dir("/tmp") {
		t.Fatalf("fallback expected root, got %q", got)
	}
	_ = context.Background()
}
