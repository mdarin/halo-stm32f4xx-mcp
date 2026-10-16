# H.A.L.O. STM32F407 MCP — build/test/e2e
# Rust toolkit (toolkit/) + Go MCP server (mcp-server/)

TOOLKIT_DIR := toolkit
SERVER_DIR  := mcp-server
SERVER_BIN  := $(SERVER_DIR)/mcp-server
TOOLKIT_BIN := $(TOOLKIT_DIR)/target/release/toolkit

.PHONY: build test e2e smoke clean

build: build-toolkit build-server
	@echo "OK: $(TOOLKIT_BIN), $(SERVER_BIN)"

build-toolkit:
	cargo build --release --manifest-path $(TOOLKIT_DIR)/Cargo.toml

build-server:
	go build -C $(SERVER_DIR) -o mcp-server .

test: test-rust test-go

test-rust:
	cargo test --manifest-path $(TOOLKIT_DIR)/Cargo.toml

test-go:
	go test -C $(SERVER_DIR) ./...

# Полный e2e: сервер + toolkit через STDIO
e2e: build
	TOOLKIT_PATH=$(TOOLKIT_BIN) MCP_SERVER_PATH=$(SERVER_BIN) python3 scripts/e2e_advanced_tools.py
	MCP_SERVER_PATH=$(SERVER_BIN) python3 scripts/test_mcp_server.py

# Быстрый smoke без пересборки
smoke:
	TOOLKIT_PATH=$(TOOLKIT_BIN) MCP_SERVER_PATH=$(SERVER_BIN) python3 scripts/e2e_advanced_tools.py

clean:
	rm -f $(SERVER_BIN)
	cargo clean --manifest-path $(TOOLKIT_DIR)/Cargo.toml
