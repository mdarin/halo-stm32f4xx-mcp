# H.A.L.O. — Доска задач

## 📋 Инструкция

**Назначение:** Этот файл — рабочая доска для отслеживания текущих задач. НЕ добавляйте сюда весь план из TASK.md.

**Правила работы:**
1. **Одна задача за раз** — в разделе "Текущая задача" только один активный блок
2. **Декомпозиция** — каждая задача разбита на пункты (чек-лист)
3. **Отметки о выполнении** — используйте `[x]` для выполненных пунктов
4. **Статус задачи** — меняйте в шапке задачи: `🟡 В процессе` | `🟢 Завершена` | `🔴 Заблокирована`
5. **Переход к следующей** — после завершения задачи перемещайте её в "Архив", а новую — в "Текущая задача"

**Приоритеты (порядок реализации):**
1. **Критичный путь** — без этого система не работает (парсинг PDF, базовые CubeMX tools, RAG)
2. **Важно** — резко повышает качество (find_weak, validate, retry)
3. **Nice to have** — можно отложить (J-Link integration, SSE transport)

**Связь с TASK.md:**
- `TASK.md` — полный план архитектуры и реализации (~1500 строк)
- `BOARD.md` — рабочая доска для текущей задачи (этот файл)
- Перед началом работы прочитайте соответствующую фазу из TASK.md

---

## 🎯 Текущая задача

*(Нет текущих задач — MVP завершён)*

---

## 📦 Архив выполненных задач

### Задача #3: MCP Server на Go (6 инструментов) ✅ ЗАВЕРШЕНА
**Статус:** 🟢 Завершена (MVP)
**Приоритет:** Критичный путь
**Срок:** 5-7 дней (фактически: 2 дня)
**Связь с TASK.md:** Фаза 2, Шаги 2.1-2.7
**Дата начала:** 2026-10-14
**Дата завершения:** 2026-10-15

**Цель:** Создать MCP сервер на Go с инструментами для агента.

**Выполнено (100%):**

- [x] **3.1 Структура Go проекта** (с MCP SDK v1.8.0)
  - `mcp-server/go.mod` — Go 1.27 + `github.com/modelcontextprotocol/go-sdk v1.8.0`
  - `mcp-server/main.go` — с официальным MCP SDK (`mcp.NewServer`, `mcp.AddTool`, `mcp.StdioTransport`)
  - `mcp-server/internal/config/config.go` — конфигурация (env vars)
  - `mcp-server/internal/rag/qdrant/client.go` — Qdrant HTTP клиент (`with_payload: true`)
  - `mcp-server/internal/rag/embedder/client.go` — Ollama embedder (retry x3)
  - `mcp-server/internal/rag/searcher/searcher.go` — RAG searcher (vector + filters)
  - `mcp-server/internal/cubemx/parser.go` — парсер USER CODE блоков
  - `mcp-server/internal/cubemx/patcher.go` — безопасная вставка кода
  - `mcp-server/internal/cubemx/finder.go` — поиск weak function overrides
  - `mcp-server/internal/cubemx/validator.go` — валидатор проекта

- [x] **3.2 MCP tool `query_hardware_docs`** (v0.1.0)
  - Входные параметры: `query`, `peripheral` (optional), `limit` (optional)
  - Выход: `SearchResult` (text, source, peripheral, chapter, section, title, score)
  - Тесты: 4/4 пройдены (score 0.57-0.75)

- [x] **3.3 MCP tool `check_f407_rules`** (v0.2.0)
  - Проверка C кода на критические ошибки F407
  - 4 правила: CCM+DMA, NVIC priority, USB 48MHz, Flash ROP Level 2
  - Тесты: 4/4 пройдены

- [x] **3.4 MCP tool `cubemx_read_user_blocks`** (v0.3.0)
  - Чтение USER CODE блоков из CubeMX проекта
  - Парсинг одного файла или всего проекта
  - Тесты: ShiftCommander (173 блока в 9 файлах)

- [x] **3.5 MCP tool `cubemx_insert_code`** (v0.4.0)
  - Безопасная вставка кода в USER CODE блоки
  - Режимы: append, replace
  - Атомарная запись, валидация BEGIN/END

- [x] **3.6 MCP tool `cubemx_find_weak_overrides`** (v0.5.0)
  - Поиск переопределённых HAL weak функций
  - Сканирует все .c файлы проекта

- [x] **3.7 MCP tool `cubemx_validate_project`** (v0.5.0)
  - Валидация целостности CubeMX проекта
  - Проверка парности USER CODE блоков
  - Обнаружение дубликатов weak функций

- [x] **3.8 STDIO транспорт** (через MCP SDK)
  - `mcp.StdioTransport` из официального MCP SDK v1.8.0
  - JSON-RPC 2.0 (полная совместимость с MCP spec)

- [x] **3.9 Документация** (`mcp-server/README.md`, `docs/INTEGRATION.md`)
  - Quick start, конфигурация, примеры использования
  - Интеграция с Qwen Code, Cursor, Cline

**Результат:**
- ✅ Go binary `mcp-server.exe` (13 MB, v0.5.0)
- ✅ 6 MCP инструментов готовы к использованию
- ✅ Тесты STDIO protocol пройдены
- ✅ Документация для интеграции с AI-агентами

**Git commits:**
- `744117d` — Complete MVP: 6/6 tools, v0.5.0
- `114ac1b` — Add cubemx_insert_code (v0.4.0)
- `212a3b5` — Add cubemx_read_user_blocks (v0.3.0)
- `e2ebae3` — Add check_f407_rules (v0.2.0)
- `3081157` — Add MCP server documentation
- `24a3f8a` — Refactor to use official MCP SDK v1.8.0
- `633cb27` — Add Go MCP server skeleton (v0.1.0)

**Интеграция:**
- [ ] **3.13 Интеграция с Cursor/Cline** — добавить MCP сервер в `.cursor/mcp.json`

**Критерии готовности (MVP):**
- ✅ `query_hardware_docs` работает через STDIO
- ✅ Поиск возвращает релевантные результаты (score > 0.5)
- ✅ Фильтры по peripheral работают
- ✅ Метаданные возвращаются (title, source, peripheral)
- ✅ Тесты STDIO protocol пройдены (4/4)
- ⏳ Интеграция с Cursor/Cline проверена
- ⏳ CubeMX tools реализованы

**Результат (текущий):**
- ✅ Go binary `mcp-server.exe` (13 MB, с MCP SDK v1.8.0)
- ✅ MCP tool `query_hardware_docs` готов к использованию
- ✅ Агент может искать по документации RM0090
- ✅ Документация: `mcp-server/README.md`

**Git commits:**
- `3081157` — Add comprehensive MCP server documentation (README.md)
- `1d2f6e6` — Fix Qdrant search to return payload metadata + add MCP server tests
- `24a3f8a` — Refactor MCP server to use official MCP SDK v1.8.0
- `633cb27` — Add Go MCP server skeleton with query_hardware_docs tool

**Известные проблемы:**
- ⚠️ `golangci-lint` v2.1.6 несовместим с Go 1.27.1 (используем `go vet` + `gofmt`)
- ⚠️ MSYS2 Go (1.25.3) конфликтует с официальным Go (1.27.1) — используем `set PATH=C:\Program Files\Go\bin;%PATH%`

---

## 📦 Архив выполненных задач

### Задача #2.5: Создание f407_known_issues.md ✅ ЗАВЕРШЕНА
**Статус:** 🟢 Завершена
**Приоритет:** Важно
**Срок:** 1 час
**Дата завершения:** 2026-10-14

**Что сделано:**
- [x] Создан `kb/hardware_pitfalls/f407_known_issues.md` (295 строк)
- [x] 6 критических проблем F407:
  1. CCM RAM + DMA — НЕСОВМЕСТИМО
  2. NVIC Priority Grouping
  3. USB/SDIO требуют 48 MHz
  4. Таймеры APB1/APB2 умножают частоту
  5. Boot configuration
  6. Flash write protection (ROP)
- [x] YAML frontmatter для RAG
- [x] Проиндексирован в Qdrant (точка 364)
- [x] Тест поиска: "DMA CCM RAM" → найден (score 0.6725)

**Git commit:** `d58bb1f`

---

### Задача #2: Индексация в Qdrant ✅ ЗАВЕРШЕНА
**Статус:** 🟢 Завершена
**Приоритет:** Критичный путь
**Срок:** 0.5 дня (фактически: ~4 часа с отладкой)
**Дата завершения:** 2026-10-14

**Что сделано:**
- [x] Создан `docker-compose.yml` (только Qdrant)
- [x] Создан `scripts/index_docs.py` (индексация 363 файлов)
- [x] Создан `scripts/enrich_manifest.py` (обогащение peripheral/registers/tags)
- [x] Создано виртуальное окружение Python (`venv/`)
- [x] Запущен Qdrant в Docker (порт 6333)
- [x] Запущена индексация (363 точки → 364 с known_issues)
- [x] Проверено качество поиска (score 0.61-0.75)
- [x] Создан `scripts/qdrant_backup.py` (backup/restore)
- [x] Создан `backups/BACKUP.md` (документация backup системы)

**Результат:**
- ✅ Коллекция `stm32f407_docs` создана (364 точки)
- ✅ Все F407-relevant файлы проиндексированы
- ✅ Поиск работает (векторный + фильтры по peripheral/registers/tags)
- ✅ RAG готов к использованию
- ✅ Backup система работает (1 backup создан, 9.05 MB)

**Git commits:** `611977a`, `8722ed0`, `0339e75`

---

### Задача #1: Парсинг PDF документации через Marker ✅ ЗАВЕРШЕНА
**Статус:** 🟢 Завершена
**Приоритет:** Критичный путь
**Срок:** 2-3 дня (фактически: выполнено ранее)
**Дата завершения:** 2026-10-14

**Что сделано:**
- [x] Распознан RM0090 (Reference Manual, ~1749 страниц)
- [x] Создан `kb/manifest.json` (428 записей с метаданными)
- [x] Создан `kb/index.json` (машинный индекс по главам)
- [x] Создан `kb/INDEX.md` (семантический указатель "задача → глава")
- [x] Создан `kb/INGEST.md` (инструкция по индексации)
- [x] Разбиение на секции (главы 1-39, 73 файла помечены not_on_f407)

**Результат:**
- ✅ 431 MD файл в `kb/` (428 секций + INDEX.md + INGEST.md + SPLIT_REPORT.md)
- ✅ Готово к индексации в Qdrant

---

## 📝 Заметки

**2026-10-15 (обновлено):**
- ✅ Задача #1 завершена: парсинг PDF (431 MD файл в kb/)
- ✅ Задача #2 завершена: индексация в Qdrant (364 точки)
- ✅ Задача #2.5 завершена: f407_known_issues.md (6 проблем, проиндексирован)
- ✅ Задача #3 (70%): MCP Server на Go
  - ✅ `query_hardware_docs` работает (MCP SDK v1.8.0, тесты 4/4)
  - ⏳ CubeMX tools (read_user_blocks, insert_code, check_f407_rules)
  - ⏳ Интеграция с Cursor/Cline

**Стек технологий:**
- **Qdrant** v1.19.0 (Docker, REST :6333, gRPC :6334, bind mount)
- **Ollama** (nomic-embed-text, 768 dims, localhost:11434)
- **Python 3.12** (venv, requests) — для скриптов индексации
- **Go 1.27.1** — для MCP сервера (с MCP SDK v1.8.0)
- **MCP SDK** v1.8.0 (`github.com/modelcontextprotocol/go-sdk`)

**Ссылки:**
- Полный план: `TASK.md`
- База знаний: `kb/` (432 MD файла)
- MCP сервер: `mcp-server/` (Go, README.md)
- Скрипты: `scripts/` (enrich_manifest.py, index_docs.py, qdrant_backup.py, test_mcp_server.py)
- Docker: `docker-compose.yml` (Qdrant)
- Backup: `backups/BACKUP.md` (инструкция)

**Известные проблемы:**
- ⚠️ Docker Desktop требует ручного запуска на Windows
- ⚠️ MSYS2 Python блокирует pip (PEP 668) — используется venv
- ⚠️ MSYS2 Go (1.25.3) конфликтует с официальным Go (1.27.1) — используем `set PATH=C:\Program Files\Go\bin;%PATH%`
- ⚠️ `golangci-lint` v2.1.6 несовместим с Go 1.27.1 — используем `go vet` + `gofmt`
