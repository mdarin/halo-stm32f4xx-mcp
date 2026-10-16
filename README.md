# H.A.L.O. STM32F407 MCP

MCP-сервер для STM32F407-разработки: поиск по документации (RAG), статический
анализ DMA/CCM-RAM практик и CubeMX-инструменты. Состоит из двух компонентов:

- **`mcp-server/`** — Go MCP-сервер (STDIO), 9 инструментов
- **`toolkit/`** — Rust CLI (tree-sitter AST, lint F407, doc-gen, rag-query/search/upsert/index)
  — единственный владелец RAG-доступа; подключается сервером через `exec.Command`

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
| `TOOLKIT_PATH` | Путь к бинарнику Rust toolkit; без него 4 toolkit-инструмента (включая `query_hardware_docs`) не регистрируются | — |
| `QDRANT_URL` | Qdrant (читает toolkit; Go-сервер RAG-доступа не имеет с Ф2-1) | `http://127.0.0.1:6333` |
| `QDRANT_API_KEY` | API-ключ Qdrant (если настроен) | — |
| `QDRANT_COLLECTION` | Коллекция с документацией | `stm32f407_docs` |
| `OLLAMA_URL` | Ollama для эмбеддингов | `http://127.0.0.1:11434` |
| `EMBED_MODEL` | Модель эмбеддингов | `nomic-embed-text` |

Toolkit при запуске читает те же `QDRANT_URL`/`QDRANT_API_KEY`/`OLLAMA_URL`
(см. `toolkit/CONTRACT.md` §5). Контракт toolkit↔сервер: `toolkit/CONTRACT.md`.

## Инструменты MCP

Чистый Go: `check_f407_rules`, `cubemx_read_user_blocks`, `cubemx_insert_code`,
`cubemx_find_weak_overrides`, `cubemx_validate_project`.

Через Rust toolkit (`TOOLKIT_PATH` обязателен):
`query_hardware_docs` — поиск по документации (`rag-query`); `ast_analyze_code`
— AST-анализ C-кода (функции, переменные, HAL-вызовы, атрибуты, точные
byte/line-позиции); `advanced_lint` — правила F407 (DMA-буфер в CCMRAM и др.,
опционально flow-анализ); `generate_doc_comments` — Doxygen-комментарии для
функций без документации.

Пополнение базы знаний из Rust-стороны: `toolkit rag-index --path kb`
(по manifest.json, детерминированные id — совместимо с `scripts/index_docs.py`)
или низкоуровневый `toolkit rag-upsert` (точки по stdin).

## Индексация документации

```bash
python3 -m venv /tmp/venv-index && /tmp/venv-index/bin/pip install -q pypdf requests
/tmp/venv-index/bin/python scripts/index_docs.py   # kb/stm32f4xx/{reference,datasheet,errata}/ -> Qdrant
```
