# H.A.L.O. — Доска задач

## 📋 Инструкция

**Назначение:** Этот файл — рабочая доска для отслеживания текущих задач. НЕ добавляйте сюда весь план из TASK.md.

**Правила работы:**
1. **Одна задача за раз** — в разделе "Текущая задача" только один активный блок
2. **Декомпозиция** — каждая задача разбита на пункты (чек-лист)
3. **Отметки о выполнении** — используйте `[x]` для выполненных пунктов
4. **Статус задачи** — меняйте в шапке задачи: `🟡 В процессе` | `🟢 Завершена` | `🔴 Заблокирована`
5. **Переход к следующей** — после завершения задачи перемещайте её в "Архив", а новую — в "Текущая задача"

---

## 🎯 Текущая задача

*(Нет текущих задач — все фазы до Ф5 включительно завершены)*

**Следующая кандидатура (из ROADMAP.md):**
- Фаза 1: J-Link интеграция (`flash_firmware`, `read_rtt_logs`, `debug_attach`)
- Фаза 4: Build-интеграция (`build_project`, `analyze_build_errors`)

---

## 📊 Текущее состояние проекта (2026-10-18)

| Компонент | Версия | Статус |
|---|---|---|
| MCP Server (Go) | v0.7.0 | ✅ 12 инструментов |
| Rust Toolkit | v1.1.0 | ✅ AST, RAG, context |
| Qdrant коллекция | stm32f407_docs | ✅ 409 точек (green) |
| KB файлов | 402 (F407-relevant) | ✅ reference + datasheet + errata |
| RAG поиск | hybrid (vector + BM25) | ✅ протестирован |

**Архитектура:** Go MCP server → Rust CLI toolkit (`exec.Command`) → Qdrant/Ollama

**12 инструментов MCP:**
- Base (Go): `check_f407_rules`, `cubemx_read_user_blocks`, `cubemx_insert_code`, `cubemx_find_weak_overrides`, `cubemx_validate_project`
- Advanced (Rust toolkit): `query_hardware_docs`, `ast_analyze_code`, `advanced_lint`, `generate_doc_comments`, `project_context_load`, `project_context_save`, `project_context_search`

---

## 📦 Архив выполненных задач

### Задача #6: Расширение KB + пересборка MCP ✅ ЗАВЕРШЕНА
**Статус:** 🟢 Завершена
**Дата:** 2026-10-18
**Связь с ROADMAP.md:** Фаза 2 (Расширение базы знаний)

**Что сделано:**
- [x] KB расширена: reference (355) + datasheet (38) + errata (9) = 402 файла
- [x] Rust toolkit v1.1.0 пересобран
- [x] Go MCP server v0.7.0 пересобран (12 инструментов)
- [x] Qdrant коллекция пересоздана с hybrid search (vector + BM25 sparse)
- [x] Индексация: 409 точек (362 reference + 38 datasheet + 9 errata)
- [x] RAG поиск протестирован ("DMA buffer CCM RAM" → 3 результата, score 0.63-0.65)
- [x] Тестирование на ShiftCommander: analyze, lint, rag-query

**Исправления toolkit:**
- [x] Добавлен вывод ошибок при индексации (`eprintln!` в `rag/mod.rs`)
- [x] Созданы batch-файлы для Windows (обходной путь для env vars)

**Git commit:** `d12a3b1` — fix(toolkit): вывод ошибок при индексации kb (read/upsert)

---

### Задача #5: Фаза 5 — Контекстная память агента ✅ ЗАВЕРШЕНА
**Статус:** 🟢 Завершена
**Дата:** 2026-10-17
**Связь с TASK.md:** Фаза 5, Шаги 5.1-5.7

**Что сделано:**
- [x] Отдельная Qdrant коллекция `stm32_project_context`
- [x] MCP tool `project_context_save` — сохранение записей (key, text, периферия/IRQ/prefs)
- [x] MCP tool `project_context_load` — загрузка контекста проекта
- [x] MCP tool `project_context_search` — семантический поиск по контексту
- [x] Авто-лог модификаций `cubemx_insert_code` → ctx-save (file, block, mode, bytes, timestamp)
- [x] Toolkit CLI: `ctx-save`, `ctx-load`, `ctx-search`, `ctx-extract`
- [x] `ctx-extract` — извлечение периферии из .ioc, IRQ из AST, USER CODE блоки
- [x] E2E тесты: `e2e_context.py`

**Результат:** Агент запоминает между сессиями — какая периферия настроена, что менял, предпочтения пользователя.

**Git commits:** `6d73617`, `fedad6c`, `5a6f9aa`

---

### Задача #4: Фаза 2 — Rust Toolkit + AST анализ ✅ ЗАВЕРШЕНА
**Статус:** 🟢 Завершена
**Дата:** 2026-10-16
**Связь с TASK.md:** Фаза 2 (v2), ROADMAP Фаза 3

**Что сделано:**
- [x] Rust CLI toolkit (`toolkit/`) — отдельный бинарник
- [x] tree-sitter AST парсинг C кода (функции, переменные, HAL-вызовы, атрибуты)
- [x] MCP tool `ast_analyze_code` — AST-анализ с точными byte/line-позициями
- [x] MCP tool `advanced_lint` — правила F407 (DMA-буфер в CCMRAM и др., опционально flow-анализ)
- [x] MCP tool `generate_doc_comments` — Doxygen-комментарии для функций без документации
- [x] RAG через toolkit: `rag-query`, `rag-search`, `rag-upsert`, `rag-index`
- [x] Go MCP server вызывает toolkit через `exec.Command` (JSON stdin/stdout)
- [x] Контракт toolkit↔сервер: `toolkit/CONTRACT.md`

**Результат:** Go MCP — тонкий интерфейс, Rust toolkit — тяжёлая lifting (AST, RAG, анализ).

**Архитектурное решение:** CLI (не gRPC/FFI) — benchmark показал что tokio+reqwest медленнее ureq на +28%. Узкое место — CPU-bound эмбеддинг в Ollama, конкурентность не помогает.

---

### Задача #3: MCP Server на Go (6 инструментов) ✅ ЗАВЕРШЕНА
**Статус:** 🟢 Завершена (MVP)
**Дата:** 2026-10-14 — 2026-10-15

**Результат:** Go binary `mcp-server.exe` (v0.5.0), 6 MCP инструментов, STDIO транспорт через MCP SDK v1.8.0.

**Инструменты:** `query_hardware_docs` (позже перенесён в toolkit), `check_f407_rules`, `cubemx_read_user_blocks`, `cubemx_insert_code`, `cubemx_find_weak_overrides`, `cubemx_validate_project`.

---

### Задача #2.5: Создание f407_known_issues.md ✅ ЗАВЕРШЕНА
**Дата:** 2026-10-14 | **Commit:** `d58bb1f`

6 критических проблем F407 в `kb/hardware_pitfalls/f407_known_issues.md`.

---

### Задача #2: Индексация в Qdrant ✅ ЗАВЕРШЕНА
**Дата:** 2026-10-14 | **Коллекция:** `stm32f407_docs` (364 точки изначально)

Qdrant в Docker, `scripts/index_docs.py`, `scripts/enrich_manifest.py`.

---

### Задача #1: Парсинг PDF документации ✅ ЗАВЕРШЕНА
**Дата:** 2026-10-14 | **KB:** 432 MD файла

RM0090 (1749 страниц) → `kb/stm32f4xx/reference/` + datasheet + errata.

---

## 📝 Заметки

**Стек технологий (актуально 2026-10-18):**
- **Go 1.27.1** — MCP сервер (MCP SDK v1.8.0)
- **Rust (stable)** — toolkit (tree-sitter, ureq, sha2)
- **Qdrant** v1.19.0 (Docker, REST :6333, gRPC :6334, hybrid search)
- **Ollama** (nomic-embed-text, 768 dims, localhost:11434)
- **Python 3.12** — только для `scripts/index_docs.py` (venv)

**Сборка:**
```bash
make build          # cargo build --release + go build
make test           # cargo test + go test
```

**Запуск MCP:**
```batch
set TOOLKIT_PATH=toolkit\target\release\toolkit.exe
set QDRANT_URL=http://localhost:6333
set OLLAMA_URL=http://localhost:11434
mcp-server\mcp-server.exe
```

**Windows: переменные окружения для toolkit** — `set VAR=value` в cmd.exe не передаётся в subprocess. Используются batch-файлы (`index_*.bat`, `test_*.bat`).

**Известные проблемы:**
- ⚠️ Go тесты не работают на Windows (stub-скрипты `.sh`, `stubScript` удаляет кавычки из JSON в `.bat`)
- ⚠️ Docker Desktop требует ручного запуска на Windows
- ⚠️ `golangci-lint` v2.1.6 несовместим с Go 1.27.1 — используем `go vet` + `gofmt`
- ⚠️ MSYS2 Go (1.25.3) конфликтует с официальным Go (1.27.1)
