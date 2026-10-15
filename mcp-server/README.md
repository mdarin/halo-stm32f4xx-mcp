# STM32F407 MCP Server

MCP (Model Context Protocol) сервер для работы с документацией STM32F407 через RAG-поиск по базе знаний RM0090.

## 📊 Статус (2026-10-15)

**Готовность:** 70% (MVP)

**Реализовано:**
- ✅ MCP tool `query_hardware_docs` — поиск по документации RM0090
- ✅ Qdrant клиент (HTTP API, с `with_payload=true`)
- ✅ Ollama embedder (nomic-embed-text, 768 dims, retry x3)
- ✅ RAG searcher (векторный поиск + фильтры по peripheral)
- ✅ STDIO транспорт (через официальный MCP SDK v1.8.0)
- ✅ Тесты STDIO protocol (4/4 пройдены)

**В разработке:**
- ⏳ Интеграция с Cursor/Cline
- ⏳ Дополнительные инструменты (cubemx_read_user_blocks, check_f407_rules)

## 🚀 Быстрый старт

### Компиляция

```bash
cd mcp-server
set PATH=C:\Program Files\Go\bin;%PATH%
go build -o mcp-server.exe .
```

**Примечание:** На этой Windows-машине два Go компилятора в PATH. MSYS2 Go (1.25.3) конфликтует с официальным Go (1.27.1). Используйте `set PATH=C:\Program Files\Go\bin;%PATH%` или полный путь `"C:\Program Files\Go\bin\go.exe"`.

### Запуск

```bash
./mcp-server.exe
```

Сервер запустится и будет слушать STDIO. Логи пишутся в STDERR.

### Тестирование

```bash
cd ..
venv\bin\python.exe scripts/test_mcp_server.py
```

**Тесты:**
1. `initialize` — проверка инициализации сервера
2. `tools/list` — список зарегистрированных инструментов
3. `query_hardware_docs` (GPIO MODER) — проверка поиска
4. `query_hardware_docs` с фильтром peripheral=TIM — проверка фильтров

## 🔧 Конфигурация

Через переменные окружения:

| Переменная | По умолчанию | Описание |
|------------|--------------|----------|
| `QDRANT_URL` | `http://localhost:6333` | URL Qdrant API |
| `OLLAMA_URL` | `http://localhost:11434` | URL Ollama API |
| `COLLECTION` | `stm32f407_docs` | Имя коллекции в Qdrant |
| `EMBED_MODEL` | `nomic-embed-text` | Модель для эмбеддингов |

## 📚 MCP Tools

### `query_hardware_docs`

Поиск по документации STM32F407 (RM0090) для регистров, периферии, тактирования и т.д.

**Входные параметры:**
- `query` (string, required) — поисковый запрос (например, "GPIO MODER register", "TIM1 BDTR dead-time")
- `peripheral` (string, optional) — фильтр по типу периферии (GPIO, TIM, DMA, UART, SPI, I2C, ADC, CAN, USB, etc.)
- `limit` (integer, optional) — максимальное количество результатов (по умолчанию: 5)

**Выход:**
```json
{
  "query": "GPIO MODER register",
  "peripheral": "GPIO",
  "count": 3,
  "results": [
    {
      "text": "...",
      "source": "08_8_4_GPIO_registers.md",
      "peripheral": "GPIO",
      "chapter": 8,
      "section": "8.4",
      "title": "GPIO registers",
      "score": 0.7481
    }
  ]
}
```

**Примеры использования:**

```json
// Поиск информации о GPIO MODER
{
  "query": "GPIO MODER register configuration",
  "limit": 3
}

// Поиск только по таймерам
{
  "query": "PWM configuration",
  "peripheral": "TIM",
  "limit": 5
}

// Поиск информации о DMA
{
  "query": "DMA CCM RAM incompatibility"
}
```

## 🏗️ Архитектура

```
mcp-server/
├── main.go                          # MCP сервер с официальным SDK v1.8.0
├── go.mod                           # Go 1.27 + MCP SDK
├── go.sum                           # Зависимости
├── internal/
│   ├── config/config.go             # Конфигурация (env vars)
│   └── rag/
│       ├── qdrant/client.go         # Qdrant HTTP клиент (search)
│       ├── embedder/client.go       # Ollama embedder (retry x3)
│       └── searcher/searcher.go     # RAG searcher (vector + filters)
└── mcp-server.exe                   # Binary (13 MB, в .gitignore)
```

**Ключевые компоненты:**

1. **MCP SDK v1.8.0** (`github.com/modelcontextprotocol/go-sdk`)
   - Официальный SDK от Model Context Protocol
   - `mcp.NewServer`, `mcp.AddTool`, `mcp.StdioTransport`
   - Полная совместимость с MCP spec (JSON-RPC 2.0)

2. **Qdrant клиент**
   - HTTP API (REST)
   - Поиск с `with_payload=true` для возврата метаданных
   - Поддержка фильтров по полям (peripheral, registers, tags)

3. **Ollama embedder**
   - Модель `nomic-embed-text` (768 dims)
   - Retry-логика (3 попытки с exponential backoff)
   - Эндпоинт: `/api/embed`

4. **RAG Searcher**
   - Комбинирует embedder + Qdrant
   - Векторный поиск (cosine similarity)
   - Фильтры по peripheral (опционально)

## 🧪 Тестирование

### Ручное тестирование через STDIO

```bash
# Запустить сервер
./mcp-server.exe

# В другом терминале отправить JSON-RPC запрос
echo '{"jsonrpc":"2.0","method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"0.1.0"}},"id":1}' | ./mcp-server.exe

# Список инструментов
echo '{"jsonrpc":"2.0","method":"tools/list","params":{},"id":2}' | ./mcp-server.exe

# Поиск
echo '{"jsonrpc":"2.0","method":"tools/call","params":{"name":"query_hardware_docs","arguments":{"query":"GPIO MODER","limit":3}},"id":3}' | ./mcp-server.exe
```

### Автоматическое тестирование

```bash
venv\bin\python.exe scripts/test_mcp_server.py
```

**Результаты тестов (2026-10-15):**
```
✅ PASS: initialize (сервер: stm32f407-mcp v0.1.0)
✅ PASS: tools/list (1 инструмент: query_hardware_docs)
✅ PASS: query_hardware_docs (GPIO MODER, score: 0.75)
✅ PASS: query with filter (peripheral=TIM, все результаты с TIM)

Total: 4/4 tests passed
```

## 📊 Качество поиска

**Тестовые запросы (score 0.57-0.75):**

| Запрос | Лучший результат | Score |
|--------|------------------|-------|
| "GPIO MODER register" | `08_8_4_GPIO_registers.md` | 0.7481 |
| "TIM1 BDTR dead-time" | `17_17_4_18_TIM1_and_TIM8_break_and_deadtime_register_TIM.md` | 0.6350 |
| "DMA CCM RAM" | `kb/hardware_pitfalls/f407_known_issues.md` | 0.6725 |
| "PWM + filter TIM" | `18_18_3_9_PWM_mode.md` | 0.6133 |

## ⚠️ Известные проблемы

### golangci-lint несовместим с Go 1.27.1

**Проблема:** `golangci-lint` v2.1.6 (последняя версия) выдаёт ошибку:
```
export data version 4 is greater than maximum supported version 2
```

**Решение:** Использовать стандартные Go инструменты:
```bash
go vet ./...       # Статический анализ
gofmt -l .         # Проверка форматирования
gofmt -w .         # Применение форматирования
```

### MSYS2 Go конфликт

**Проблема:** Два Go компилятора в PATH:
- `C:\msys64\mingw64\bin\go.exe` — Go 1.25.3 (MSYS2, первый в PATH)
- `C:\Program Files\Go\bin\go.exe` — Go 1.27.1 (официальный, второй в PATH)

**Решение:**
```bash
# Временно изменить PATH
set PATH=C:\Program Files\Go\bin;%PATH%

# Или использовать полный путь
"C:\Program Files\Go\bin\go.exe" build -o mcp-server.exe .
```

## 🔗 Связанные документы

- **TASK.md** — полный план архитектуры и реализации
- **BOARD.md** — рабочая доска задач
- **kb/** — база знаний (432 MD файла)
- **backups/BACKUP.md** — инструкция по backup Qdrant

## 📝 Git commits

**Последние commits:**
```
1d2f6e6 — Fix Qdrant search to return payload metadata + add MCP server tests
24a3f8a — Refactor MCP server to use official MCP SDK v1.8.0
633cb27 — Add Go MCP server skeleton with query_hardware_docs tool
```

---

**Последнее обновление:** 2026-10-15  
**Статус:** 70% завершено, `query_hardware_docs` полностью работает  
**Следующий шаг:** Интеграция с Cursor/Cline, добавление дополнительных инструментов
