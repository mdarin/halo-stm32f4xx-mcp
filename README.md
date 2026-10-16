# H.A.L.O. STM32F407 MCP

MCP-сервер для STM32F407-разработки: поиск по документации (RAG), статический
анализ DMA/CCM-RAM практик и CubeMX-инструменты. Состоит из двух компонентов:

- **`mcp-server/`** — Go MCP-сервер (STDIO), 9 инструментов
- **`toolkit/`** — Rust CLI (tree-sitter AST, lint F407, doc-gen, rag-search),
  подключается сервером через `exec.Command`

## Сборка

Требуется Go 1.27+, Rust (stable), Python 3 (для e2e-скриптов).

```bash
make build          # cargo build --release + go build
make test           # cargo test + go test
make e2e            # сборка + e2e через STDIO MCP-клиент
```

## Запуск

```bash
TOOLKIT_PATH=toolkit/target/release/toolkit \
QDRANT_URL=http://127.0.0.1:6333 \
OLLAMA_URL=http://127.0.0.1:11434 \
./mcp-server/mcp-server
```

## Переменные окружения

| Переменная | Назначение | По умолчанию |
|---|---|---|
| `TOOLKIT_PATH` | Путь к бинарнику Rust toolkit; без него 3 advanced-инструмента не регистрируются | — |
| `QDRANT_URL` | Qdrant для `query_hardware_docs` | `http://127.0.0.1:6333` |
| `QDRANT_API_KEY` | API-ключ Qdrant (если настроен) | — |
| `QDRANT_COLLECTION` | Коллекция с документацией | `stm32f407_docs` |
| `OLLAMA_URL` | Ollama для эмбеддингов | `http://127.0.0.1:11434` |
| `EMBED_MODEL` | Модель эмбеддингов | `nomic-embed-text` |

Toolkit при запуске читает те же `QDRANT_URL`/`QDRANT_API_KEY`/`OLLAMA_URL`
(см. `toolkit/CONTRACT.md` §5). Контракт toolkit↔сервер: `toolkit/CONTRACT.md`.

## Инструменты MCP

Базовые (чистый Go): `query_hardware_docs`, `check_f407_rules`,
`cubemx_read_user_blocks`, `cubemx_insert_code`, `cubemx_find_weak_overrides`,
`cubemx_validate_project`.

Advanced (через Rust toolkit, `TOOLKIT_PATH` обязателен):
`ast_analyze_code` — AST-анализ C-кода (функции, переменные, HAL-вызовы,
атрибуты, точные byte/line-позиции); `advanced_lint` — правила F407
(DMA-буфер в CCMRAM и др., опционально flow-анализ); `generate_doc_comments` —
Doxygen-комментарии для функций без документации.

## Индексация документации

```bash
python3 -m venv /tmp/venv-index && /tmp/venv-index/bin/pip install -q pypdf requests
/tmp/venv-index/bin/python scripts/index_docs.py   # kb/ -> Qdrant
```
