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

### Задача #3: MCP tool `query_hardware_docs` (Go)
**Статус:** 🔴 Не начата  
**Приоритет:** Критичный путь  
**Срок:** 2-3 дня  
**Связь с TASK.md:** Фаза 2, Шаг 2.6

**Цель:** Создать Go MCP tool для поиска по документации через Qdrant RAG.

**Почему это важно:**
- Агент (Cursor/Cline) будет использовать этот инструмент для ответов на вопросы о регистрах, периферии, тактировании
- Без этого инструмента агент не сможет использовать базу знаний
- Это первый MCP tool из 9 запланированных

**Пункты для реализации:**

- [ ] **3.1 Создать структуру Go проекта**
  - `mcp-server/go.mod`
  - `mcp-server/main.go`
  - `mcp-server/internal/rag/qdrant.go`
  - `mcp-server/internal/rag/embedder.go`
  - `mcp-server/internal/rag/searcher.go`
  - `mcp-server/internal/transport/stdio.go`
  - `mcp-server/internal/tools/query_docs.go`

- [ ] **3.2 Реализовать Qdrant клиент** (`internal/rag/qdrant.go`)
  - Подключение к `http://localhost:6333`
  - Метод `Search(collection, vector, limit, filters)`
  - Обработка ошибок

- [ ] **3.3 Реализовать Ollama embedder** (`internal/rag/embedder.go`)
  - Подключение к `http://localhost:11434/api/embed`
  - Модель `nomic-embed-text` (768 dims)
  - Retry-логика (3 попытки)
  - Кэширование эмбеддингов (опционально)

- [ ] **3.4 Реализовать searcher** (`internal/rag/searcher.go`)
  - Гибридный поиск (vector + BM25)
  - Фильтры по `peripheral`, `registers`, `tags`
  - Маппинг результатов в `SearchResult`

- [ ] **3.5 Реализовать MCP tool** (`internal/tools/query_docs.go`)
  - Название: `query_hardware_docs`
  - Входные параметры: `query` (string), `peripheral` (optional string), `limit` (optional int)
  - Выход: список `SearchResult` (text, peripheral, register, score, source)
  - Описание для агента: "Search STM32F407 hardware documentation (RM0090) for registers, peripherals, clock configuration, etc."

- [ ] **3.6 Реализовать STDIO транспорт** (`internal/transport/stdio.go`)
  - Чтение JSON-RPC из stdin
  - Запись ответов в stdout
  - Обработка методов: `initialize`, `tools/list`, `tools/call`

- [ ] **3.7 Тестирование**
  - Запуск: `go run .`
  - Тест через MCP Inspector или Cursor
  - Проверка поиска: "GPIO MODER" → `08_8_4_GPIO_registers.md`
  - Проверка фильтров: peripheral=TIM → только таймеры

**Критерии готовности:**
- ✅ `query_hardware_docs` работает через STDIO
- ✅ Поиск возвращает релевантные результаты (score > 0.5)
- ✅ Фильтры по peripheral работают
- ✅ Интеграция с Cursor/Cline проверена

**Результат:**
- Go binary `mcp-server.exe`
- MCP tool `query_hardware_docs` готов к использованию
- Агент может искать по документации RM0090

---

## 📦 Архив выполненных задач

### Задача #2: Индексация в Qdrant ✅ ЗАВЕРШЕНА
**Статус:** 🟢 Завершена  
**Приоритет:** Критичный путь  
**Срок:** 0.5 дня (фактически: ~4 часа с отладкой)  
**Связь с TASK.md:** Фаза 1 (Qdrant) + Фаза 2 (RAG)  
**Дата завершения:** 2026-10-14

**Что сделано:**

- [x] **2.1 Создан `docker-compose.yml`** (только Qdrant)
  - Qdrant v1.19.0 в Docker
  - REST API: localhost:6333
  - gRPC: localhost:6334
  - Volume: ./data/qdrant

- [x] **2.2 Создан `scripts/index_docs.py`**
  - Чтение `kb/manifest.json` (обогащённый peripheral/registers/tags)
  - Эмбеддинг через Ollama (nomic-embed-text, 768 dims)
  - Загрузка в Qdrant коллекцию `stm32f407_docs`
  - Фильтрация `not_on_f407 == true` (исключены F42x/43x)
  - Resume support (пропуск уже проиндексированных файлов)
  - Batch upload каждые 50 файлов
  - UTF-8 кодировка для Windows

- [x] **2.3 Создан `scripts/enrich_manifest.py`**
  - Автоматическое добавление peripheral/registers/tags в manifest.json
  - Обогащено 427 из 428 файлов (99.8%)

- [x] **2.4 Создано виртуальное окружение Python**
  - `venv/` с установленным `requests`
  - Решение проблемы MSYS2 Python (PEP 668)

- [x] **2.5 Запущен Qdrant в Docker**
  - Команда: `docker compose up -d qdrant`
  - Проверка: `curl http://localhost:6333/collections`

- [x] **2.6 Запущена индексация**
  - Команда: `venv\bin\python.exe scripts/index_docs.py`
  - Время выполнения: ~15 минут
  - Результат: **363 точки** загружены в коллекцию

- [x] **2.7 Проверено качество поиска**
  - Тест 1: "GPIO MODER register" → ✅ `08_8_4_GPIO_registers.md` (score: 0.7481)
  - Тест 2: "TIM1 BDTR dead-time" → ✅ `17_17_4_18_TIM1_and_TIM8_break_and_deadtime_register_TIM.md` (score: 0.6350)
  - Тест 3: "DMA CCM RAM" → ✅ `10_10_5_DMA_registers__c01.md` (score: 0.6527)
  - Тест 4: "PWM configuration" с фильтром peripheral=TIM → ✅ `18_18_3_9_PWM_mode.md` (score: 0.6133)

**Результат:**
- ✅ Коллекция `stm32f407_docs` создана (363 точки)
- ✅ Все F407-relevant файлы проиндексированы
- ✅ Поиск работает (векторный + фильтры по peripheral/registers/tags)
- ✅ RAG готов к использованию

---

### Задача #1: Парсинг PDF документации через Marker ✅ ЗАВЕРШЕНА
**Статус:** 🟢 Завершена  
**Приоритет:** Критичный путь  
**Срок:** 2-3 дня (фактически: выполнено ранее)  
**Связь с TASK.md:** Фаза 0, Шаг 0.1  
**Дата завершения:** 2026-10-14

**Что сделано:**

- [x] **1.1 Распознан RM0090** (Reference Manual, ~1749 страниц)
  - Инструмент: Marker (pdfplumber + LLM)
  - Результат: 428 MD файлов в `kb/`

- [x] **1.2 Создан `kb/manifest.json`**
  - 428 записей с метаданными (chapter, section, title, bytes, not_on_f407)
  - Обогащён полями peripheral/registers/tags (скрипт `enrich_manifest.py`)

- [x] **1.3 Создан `kb/index.json`**
  - Машинный индекс по главам
  - Фильтр `not_on_f407` для F42x/43x

- [x] **1.4 Создан `kb/INDEX.md`**
  - Семантический указатель "задача → глава"
  - 30 рецептов (GPIO, RCC, DMA, TIM, ADC, UART, SPI, I2C, CAN, USB, etc.)

- [x] **1.5 Создан `kb/INGEST.md`**
  - Инструкция по индексации в Qdrant
  - Рекомендации по чанкингу и фильтрации

- [x] **1.6 Разбиение на секции**
  - Главы 1-39 (главы 6, 11, 15, 16, 29, 37 помечены как not_on_f407)
  - Секции >30 КБ разбиты на чанки по `###` заголовкам
  - HTML-таблицы конвертированы в Markdown

**Результат:**
- ✅ 431 MD файл в `kb/` (428 секций + INDEX.md + INGEST.md + SPLIT_REPORT.md)
- ✅ Готово к индексации в Qdrant

---

## 📝 Заметки

**2026-10-14:**
- ✅ Задача #1 завершена: парсинг PDF (431 MD файл в kb/)
- ✅ Задача #2 завершена: индексация в Qdrant (363 точки)
- ✅ RAG работает: тестовый поиск показал отличные результаты (score 0.61-0.75)
- 🔄 Задача #3: MCP tool `query_hardware_docs` (Go) — следующая

**Стек технологий:**
- **Qdrant** v1.19.0 (Docker, REST :6333, gRPC :6334)
- **Ollama** (nomic-embed-text, 768 dims, localhost:11434)
- **Python 3.12** (venv, requests) — для скриптов индексации
- **Go 1.22+** (планируется) — для MCP сервера

**Ссылки:**
- Полный план: `TASK.md`
- База знаний: `kb/` (431 MD файл)
- Скрипты: `scripts/` (enrich_manifest.py, index_docs.py, test_search.py)
- Docker: `docker-compose.yml` (Qdrant)

**Известные проблемы:**
- ⚠️ Docker Desktop требует ручного запуска на Windows
- ⚠️ MSYS2 Python блокирует pip (PEP 668) — используется venv
- ⚠️ `f407_known_issues.md` не создан (опционально, но важно для проверки аппаратных ограничений)
