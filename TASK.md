# H.A.L.O. — Hardware-Aware Local Orchestrator

# План развертывания стека и создания MCP-сервера для STM32F407 агента

## 🎯 Целевой стек

| Компонент | Технология | Обоснование |
|-----------|------------|-------------|
| **MCP Server** | Go 1.22+ | Нативный запуск на Windows, прямой доступ к J-Link/probe-rs |
| **Vector DB** | Qdrant v1.11+ | Hybrid search (vector + BM25), gRPC, Docker |
| **Embeddings** | Ollama + nomic-embed-text | Уже установлен локально, быстро (~270MB) |
| **Транспорт** | STDIO (основной) | Для Cursor/Cline, без HTTP-overhead |
| **Парсинг PDF** | Marker / Nougat | Конвертация RM0090/Datasheet/Errata в MD |
| **Debug Probe** | J-Link + probe-rs | Прошивка, RTT-логи, отладка через SWD |
| **Хост-ОС** | Windows 10/11 | Основная среда разработки |

**Архитектурное решение:** MCP Server работает **нативно на Windows** (не в Docker), что обеспечивает:
- Прямой доступ к J-Link CLI (`JFlash.exe`, `JLinkRTTClient.exe`)
- Интеграцию с probe-rs (Cargo runner для Rust)
- Прямой доступ к USB-устройствам (без проброса через Docker)
- Упрощённую отладку самого MCP сервера

## 📋 Общая архитектура

```
┌─────────────────┐      ┌──────────────────┐      ┌─────────────────┐
│  Agent (LLM)    │◄────►│  MCP Server      │◄────►│  Qdrant         │
│  (Cursor/Cline) │ STDIO│  (Go, native)    │ gRPC │  (Docker)       │
└─────────────────┘      └──────────────────┘      └─────────────────┘
                                  │                         ▲
                                  │ HTTP                    │
                                  ▼                         │
                         ┌──────────────────┐               │
                         │  Ollama          │───────────────┘
                         │  (nomic-embed)   │  embeddings
                         │  (native)        │
                         └──────────────────┘
                                  │
                                  │ USB/SWD
                                  ▼
                         ┌──────────────────┐
                         │  J-Link/probe-rs │
                         │  (hardware)      │
                         └──────────────────┘
```

**Ключевые принципы:**
- **Безопасность**: Защита от записи вне USER CODE блоков (regex + валидация)
- **Отказоустойчивость**: Retry-логика для Ollama, health checks
- **Детерминизм**: SHA-256 для ID точек (не Python hash)
- **Версионирование**: `schema_version` в Qdrant для эволюции схемы
- **Hardware integration**: Прямой доступ к J-Link для прошивки и отладки

---

## 🎯 ФАЗА 0: Подготовка базы знаний (4-6 дней)

### Шаг 0.1. Инструменты парсинга PDF → Markdown

**Проблема:** RM0090 (2000+ страниц), Datasheet (150+ страниц), Errata (100+ страниц) содержат сложные таблицы регистров, диаграммы, перекрёстные ссылки.

**Решение — двухэтапный пайплайн:**

#### Этап A: Автоматический парсинг (Marker)

```bash
# Установка Marker (GPU-ускорение опционально, CPU тоже работает)
pip install marker-pdf

# Конвертация RM0090
marker_single path/to/RM0090.pdf output_dir/ --workers 4

# Конвертация Datasheet
marker_single path/to/DM00037053.pdf output_dir/ --workers 4

# Конвертация Errata
marker_single path/to/ES0113.pdf output_dir/ --workers 4
```

**Альтернатива:** `Nougat` (от Meta, лучше для академических документов, но медленнее).

#### Этап B: Пост-обработка (опционально, для улучшения таблиц)

Скрипт `scripts/postprocess_md.py`:
- Исправляет разорванные таблицы
- Добавляет YAML frontmatter к каждому разделу
- Валидирует структуру (заголовки, списки, код)

```python
# Пример: добавление frontmatter
import re
from pathlib import Path

def add_frontmatter(md_file, metadata):
    content = md_file.read_text(encoding='utf-8')
    frontmatter = f"""---
mcu: STM32F407
peripheral: {metadata.get('peripheral', 'unknown')}
chapter: {metadata.get('chapter', 0)}
source: {md_file.name}
schema_version: 1
---

"""
    md_file.write_text(frontmatter + content, encoding='utf-8')
```

### Шаг 0.2. Структура документации

Создайте директорию `docs/` со следующей структурой:

```
docs/
├── reference_manual/          # RM0090 (после парсинга)
│   ├── 01_introduction.md
│   ├── 02_memory_and_bus.md
│   ├── 03_gpio.md
│   ├── 04_rcc_clock.md
│   ├── 05_timers.md
│   ├── 06_dma.md
│   ├── 07_usart.md
│   ├── 08_spi.md
│   ├── 09_i2c.md
│   ├── 10_adc.md
│   └── ...
├── datasheet_f407/            # DM00037053 (после парсинга)
│   └── pinout_and_memory.md
├── errata/                    # ES0113 (после парсинга)
│   └── stm32f407_errata.md
└── hardware_pitfalls/         # Вручную (критично!)
    └── f407_known_issues.md   # "Мины": CCM+DMA, NVIC, 48MHz и т.д.
```

### Шаг 0.2. Шаблон чанка (обязательный формат)

Каждый файл должен начинаться с YAML frontmatter:

```markdown
---
mcu: STM32F407
peripheral: TIM1
register: BDTR
address: 0x40010044
reset_value: 0x00000000
type: register_description
chapter: 17
tags: [dead-time, pwm, advanced-timer]
---

# TIM1_BDTR — Break and dead-time register

## Описание
Регистр управления dead-time и защитным отключением...

## Битовые поля

### Bits 15:8 — DTG[7:0]: Dead-time generator setup
- `0x00-0x7F`: DT = DTG[7:0] × t_DTS
- `0x80-0xBF`: DT = (64 + DTG[5:0]) × t_DTS
- `0xC0-0xDF`: DT = (32 + DTG[4:0]) × 2 × t_DTS
- `0xE0-0xFF`: DT = (32 + DTG[4:0]) × 8 × t_DTS

### Bit 15 — MOE: Main output enable
- `0`: Outputs disabled
- `1`: Outputs enabled

## Примечания
- t_DTS зависит от CKD в TIMx_CR1
- MOE сбрасывается аппаратно при Break event
```

### Шаг 0.3. Скрипт индексации (Python)

Создайте `scripts/index_docs.py`:

```python
import os
import re
import hashlib
import frontmatter
import requests
from pathlib import Path

QDRANT_URL = "http://localhost:6333"
COLLECTION = "stm32f407_docs"
OLLAMA_URL = "http://localhost:11434/api/embed"
EMBED_MODEL = "nomic-embed-text"
CHUNK_SIZE = 800  # символов
SCHEMA_VERSION = 1  # Версия схемы (инкремент при изменении формата)

def create_collection():
    """Создание коллекции с hybrid search (vector + BM25)"""
    requests.put(f"{QDRANT_URL}/collections/{COLLECTION}", json={
        "vectors": {"size": 768, "distance": "Cosine"},
        "sparse_vectors": {"bm25": {}},
        "metadata": {"schema_version": SCHEMA_VERSION}
    })

def embed(text):
    """Получение эмбеддинга через Ollama с retry"""
    for attempt in range(3):
        try:
            r = requests.post(OLLAMA_URL, 
                            json={"model": EMBED_MODEL, "input": text},
                            timeout=30)
            r.raise_for_status()
            return r.json()["embeddings"][0]
        except Exception as e:
            if attempt == 2:
                raise RuntimeError(f"Ollama unavailable after 3 attempts: {e}")
            time.sleep(1 * (attempt + 1))

def chunk_markdown(md_text, metadata):
    """Разбиение по заголовкам ## с сохранением контекста"""
    chunks = []
    current = []
    for line in md_text.split('\n'):
        if line.startswith('## ') and current:
            chunks.append('\n'.join(current))
            current = [line]
        else:
            current.append(line)
    if current:
        chunks.append('\n'.join(current))
    return chunks

def deterministic_id(path, chunk_idx):
    """Детерминированный ID на основе SHA-256 (не Python hash!)"""
    hash_input = f"{path}:{chunk_idx}"
    return int(hashlib.sha256(hash_input.encode()).hexdigest()[:8], 16)

def index_file(path):
    post = frontmatter.load(path)
    metadata = dict(post.metadata)
    metadata['source'] = str(path)
    metadata['schema_version'] = SCHEMA_VERSION

    chunks = chunk_markdown(post.content, metadata)
    points = []
    for i, chunk in enumerate(chunks):
        if len(chunk.strip) < 50:
            continue
        full_text = f"{metadata.get('peripheral','')} {metadata.get('register','')} {chunk}"
        vector = embed(full_text)
        points.append({
            "id": deterministic_id(str(path), i),
            "vector": vector,
            "payload": {**metadata, "text": chunk, "chunk_id": i}
        })

    # Batch upload
    if points:
        requests.put(f"{QDRANT_URL}/collections/{COLLECTION}/points",
                     json={"points": points})

if __name__ == "__main__":
    import time
    
    print(f"Creating collection with schema v{SCHEMA_VERSION}...")
    create_collection()
    
    md_files = list(Path("docs").rglob("*.md"))
    print(f"Found {len(md_files)} markdown files")
    
    for md_file in md_files:
        print(f"Indexing {md_file}")
        try:
            index_file(md_file)
        except Exception as e:
            print(f"  ERROR: {e}")
    
    print("Done!")
```

**Важно:**
- `deterministic_id()` использует SHA-256 вместо `hash()` — нет коллизий при пересоздании
- `embed()` имеет retry-логику (3 попытки с exponential backoff)
- `SCHEMA_VERSION` позволяет отслеживать эволюцию формата чанков

**Чек-лист Фазы 0:**
- [ ] Установить Marker (`pip install marker-pdf`)
- [ ] Распознать RM0090 в MD (маркер, ~2-3 часа на CPU)
- [ ] Распознать Datasheet F407
- [ ] Распознать Errata Sheet
- [ ] Написать вручную `f407_known_issues.md` (CCM+DMA, NVIC grouping и т.д.)
- [ ] Запустить `index_docs.py`, проверить коллекцию в Qdrant
- [ ] Проверить детерминизм ID (перезапустить индексацию, убедиться нет дубликатов)

---

## 🎯 ФАЗА 1: Развёртывание инфраструктуры (0.5 дня)

### Шаг 1.1. `docker-compose.yml` (только Qdrant)

**Изменение:** MCP Server больше **не в Docker**, работает нативно на Windows.

```yaml
version: '3.8'

services:
  qdrant:
    image: qdrant/qdrant:v1.11.0
    container_name: stm32_qdrant
    restart: unless-stopped
    ports:
      - "6333:6333"   # REST
      - "6334:6334"   # gRPC
    volumes:
      - ./data/qdrant:/qdrant/storage
    environment:
      - QDRANT__SERVICE__GRPC_PORT=6334
      - QDRANT__LOG_LEVEL=INFO
```

**Примечание:** MCP Server исключён из Docker Compose — запускается нативно через `go run` или `mcp-server.exe`.

### Шаг 1.2. Команды запуска

```bash
# 1. Поднять инфраструктуру
docker compose up -d qdrant

# 2. Проверить Qdrant
curl http://localhost:6333/collections

# 3. Проиндексировать документацию
python scripts/index_docs.py

# 4. Собрать и поднять MCP
docker compose up -d --build mcp-server

# 5. Проверить MCP
curl http://localhost:8080/health
```

**Чек-лист Фазы 1:**
- [ ] `docker compose up` поднимает оба сервиса
- [ ] Qdrant доступен на `:6333`
- [ ] Ollama доступна из контейнера MCP через `host.docker.internal`
- [ ] Коллекция `stm32f407_docs` создана и содержит точки

---

## 🎯 ФАЗА 2: Создание MCP-сервера на Go (5-7 дней)

### Шаг 2.1. Структура проекта

```
mcp-server/
├── go.mod
├── go.sum
├── main.go
├── internal/
│   ├── config/
│   │   └── config.go
│   ├── transport/
│   │   └── stdio.go          # STDIO transport (для Cursor/Cline)
│   ├── tools/
│   │   ├── registry.go       # реестр всех инструментов
│   │   ├── read_user_blocks.go
│   │   ├── insert_code.go
│   │   ├── query_docs.go
│   │   ├── check_f407_rules.go
│   │   ├── find_weak_overrides.go
│   │   ├── validate_project.go
│   │   ├── flash_firmware.go      # НОВОЕ: прошивка через J-Link
│   │   ├── read_rtt_logs.go       # НОВОЕ: чтение RTT-логов
│   │   └── debug_attach.go        # НОВОЕ: подключение GDB
│   ├── cubemx/
│   │   ├── parser.go
│   │   ├── patcher.go
│   │   └── validator.go
│   ├── rag/
│   │   ├── qdrant.go
│   │   ├── embedder.go       # с retry + кэш
│   │   └── searcher.go
│   ├── rules/
│   │   └── f407.go
│   └── jlink/
│       ├── flasher.go        # НОВОЕ: обёртка над JFlash.exe
│       ├── rtt.go            # НОВОЕ: обёртка над JLinkRTTClient.exe
│       └── gdb.go            # НОВОЕ: обёртка над JLinkGDBServerCL.exe
└── pkg/
    └── mcp/
        └── protocol.go
```

**Новые компоненты (добавлено):**
- `flash_firmware.go` + `jlink/flasher.go` — прошивка через J-Link
- `read_rtt_logs.go` + `jlink/rtt.go` — чтение RTT-логов (defmt-rtt)
- `debug_attach.go` + `jlink/gdb.go` — подключение GDB-отладчика

### Шаг 2.2. `go.mod` — зависимости

```go
module github.com/yourname/stm32-mcp

go 1.22

require (
    github.com/qdrant/go-client v1.10.0
    github.com/ollama/ollama/api v0.0.0
    github.com/go-playground/validator/v10 v10.22.0
)
```

### Шаг 2.3. `main.go` — точка входа

```go
package main

import (
    "log"
    "os"

    "github.com/yourname/stm32-mcp/internal/config"
    "github.com/yourname/stm32-mcp/internal/transport"
    "github.com/yourname/stm32-mcp/internal/tools"
    "github.com/yourname/stm32-mcp/internal/rag"
    "github.com/yourname/stm32-mcp/internal/cubemx"
)

func main() {
    cfg := config.Load()

    // Инициализация компонентов
    qdrant, err := rag.NewQdrantClient(cfg.QdrantURL)
    if err != nil { log.Fatal(err) }

    embedder := rag.NewOllamaEmbedder(cfg.OllamaURL, "nomic-embed-text")
    searcher := rag.NewHybridSearcher(qdrant, embedder, "stm32f407_docs")
    parser := cubemx.NewParser()
    patcher := cubemx.NewPatcher()
    validator := cubemx.NewValidator()

    // Регистрация инструментов (9 шт.)
    registry := tools.NewRegistry()
    
    // CubeMX tools (6 шт.)
    registry.Register(tools.NewReadUserBlocks(parser))
    registry.Register(tools.NewInsertCode(patcher))
    registry.Register(tools.NewFindWeakOverrides())
    registry.Register(tools.NewValidateProject(validator))
    
    // RAG tools (2 шт.)
    registry.Register(tools.NewQueryDocs(searcher))
    registry.Register(tools.NewCheckF407Rules())
    
    // J-Link/hardware tools (3 шт.) — НОВОЕ
    registry.Register(tools.NewFlashFirmware())
    registry.Register(tools.NewReadRTTLogs())
    registry.Register(tools.NewDebugAttach())

    // Запуск транспорта
    t := transport.NewStdio(registry)
    if err := t.Run(os.Stdin, os.Stdout); err != nil {
        log.Fatal(err)
    }
}
```

**Итого: 9 инструментов** (добавлены 3 для J-Link/probe-rs).

### Шаг 2.4. Ключевой инструмент: `insert_code.go`

```go
package tools

import (
    "context"
    "fmt"
    "regexp"

    "github.com/yourname/stm32-mcp/internal/cubemx"
)

type InsertCodeTool struct {
    patcher *cubemx.Patcher
}

type InsertCodeRequest struct {
    FilePath  string `json:"file_path" validate:"required"`
    BlockName string `json:"block_name" validate:"required"` // e.g. "USER CODE BEGIN 3"
    Code      string `json:"code" validate:"required"`
    Mode      string `json:"mode"` // "append" | "replace"
}

func (t *InsertCodeTool) Name() string { return "cubemx_insert_code" }
func (t *InsertCodeTool) Description() string {
    return "Safely insert code into a CubeMX USER CODE block. " +
           "NEVER writes outside BEGIN/END markers. " +
           "Available blocks: USER CODE BEGIN 1..4 in main.c, " +
           "USER CODE BEGIN 0..N in stm32f4xx_it.c"
}

func (t *InsertCodeTool) Execute(ctx context.Context, args InsertCodeRequest) (any, error) {
    // 1. Валидация
    if !isValidBlock(args.BlockName) {
        return nil, fmt.Errorf("invalid block: %s", args.BlockName)
    }

    // 2. Чтение файла
    content, err := t.patcher.ReadFile(args.FilePath)
    if err != nil { return nil, err }

    // 3. Поиск блока регулярным выражением (УЛУЧШЕННЫЙ regex)
    //    Pattern: /\* USER CODE BEGIN 3 \*/ ... /\* USER CODE END 3 \*/
    //    Поддерживает:
    //    - /* USER CODE BEGIN 3 */ (с пробелами)
    //    - /*USER CODE BEGIN 3*/ (без пробелов)
    //    - /* USER CODE BEGIN 3 @brief */ (с атрибутами)
    blockNum := extractBlockNumber(args.BlockName)
    pattern := fmt.Sprintf(
        `(?s)/\*\s*USER CODE BEGIN %d(?:\s+@\w+)?\s*\*/(.*?)/\*\s*USER CODE END %d\s*\*/`,
        blockNum, blockNum,
    )
    re := regexp.MustCompile(pattern)

    if !re.MatchString(content) {
        return nil, fmt.Errorf("block %q not found in %s", args.BlockName, args.FilePath)
    }

    // 4. Вставка кода
    var newContent string
    if args.Mode == "replace" {
        newContent = re.ReplaceAllString(content,
            fmt.Sprintf("/* USER CODE BEGIN %d @brief */\n%s\n/* USER CODE END %d */", 
                       blockNum, args.Code, blockNum))
    } else {
        newContent = re.ReplaceAllString(content,
            fmt.Sprintf("/* USER CODE BEGIN %d$1 */\n%s\n/* USER CODE END %d */", 
                       blockNum, args.Code, blockNum))
    }

    // 5. Атомарная запись
    if err := t.patcher.WriteFileAtomic(args.FilePath, newContent); err != nil {
        return nil, err
    }

    return map[string]any{
        "status":  "success",
        "file":    args.FilePath,
        "block":   args.BlockName,
        "bytes":   len(newContent),
    }, nil
}

func isValidBlock(name string) bool {
    // Поддержка USER CODE BEGIN 1..10 (не только 1..4)
    re := regexp.MustCompile(`^USER CODE BEGIN \d+$`)
    return re.MatchString(name)
}

func extractBlockNumber(name string) int {
    re := regexp.MustCompile(`USER CODE BEGIN (\d+)`)
    matches := re.FindStringSubmatch(name)
    if len(matches) < 2 { return 0 }
    var num int
    fmt.Sscanf(matches[1], "%d", &num)
    return num
}
```

**Улучшения:**
- Regex поддерживает все форматы CubeMX (с пробелами, без, с атрибутами)
- `isValidBlock()` принимает любой номер (1..N), не только 1..4
- `extractBlockNumber()` извлекает номер блока динамически

### Шаг 2.4a. НОВОЕ: `find_weak_overrides.go`

**Проблема:** Агент может создать дубликат `__weak` функции, которая уже переопределена в другом файле.

```go
package tools

import (
    "context"
    "os"
    "path/filepath"
    "regexp"
    "strings"
)

type FindWeakOverridesTool struct{}

type WeakOverride struct {
    Function string `json:"function"`
    File     string `json:"file"`
    Line     int    `json:"line"`
}

func (t *FindWeakOverridesTool) Name() string { return "cubemx_find_weak_overrides" }
func (t *FindWeakOverridesTool) Description() string {
    return "Find all __weak function overrides in the project. " +
           "Use BEFORE creating a new override to avoid duplicates. " +
           "Returns list of functions, files, and line numbers."
}

func (t *FindWeakOverridesTool) Execute(ctx context.Context, projectRoot string) (any, error) {
    var overrides []WeakOverride
    
    // Паттерн: void FunctionName(...) {
    // Ищем ВСЕ .c файлы в проекте
    err := filepath.Walk(projectRoot, func(path string, info os.FileInfo, err error) error {
        if err != nil { return nil }
        if !strings.HasSuffix(path, ".c") { return nil }
        
        content, err := os.ReadFile(path)
        if err != nil { return nil }
        
        // Ищем переопределения __weak (без ключевого слова __weak в сигнатуре)
        // Это функции, которые переопределяют HAL-weak
        re := regexp.MustCompile(`(?m)^void\s+(\w+)\s*\([^)]*\)\s*\{`)
        lines := strings.Split(string(content), "\n")
        
        for i, line := range lines {
            matches := re.FindStringSubmatch(line)
            if len(matches) < 2 { continue }
            
            funcName := matches[1]
            
            // Проверяем, есть ли эта функция в HAL (т.е. это weak override)
            if isHalWeakFunction(funcName) {
                overrides = append(overrides, WeakOverride{
                    Function: funcName,
                    File:     path,
                    Line:     i + 1,
                })
            }
        }
        
        return nil
    })
    
    if err != nil {
        return nil, err
    }
    
    return map[string]any{
        "overrides": overrides,
        "count":     len(overrides),
    }, nil
}

func isHalWeakFunction(name string) bool {
    // Список типичных HAL weak функций
    halWeakFuncs := []string{
        "HAL_GPIO_MspInit",
        "HAL_GPIO_MspDeInit",
        "HAL_TIM_Base_MspInit",
        "HAL_TIM_Base_MspDeInit",
        "HAL_UART_MspInit",
        "HAL_UART_MspDeInit",
        "HAL_SPI_MspInit",
        "HAL_SPI_MspDeInit",
        "HAL_I2C_MspInit",
        "HAL_I2C_MspDeInit",
        "HAL_ADC_MspInit",
        "HAL_ADC_MspDeInit",
        "HAL_CAN_MspInit",
        "HAL_CAN_MspDeInit",
        // Добавьте другие по необходимости
    }
    
    for _, f := range halWeakFuncs {
        if f == name { return true }
    }
    return false
}
```

**Использование:** Агент вызывает этот инструмент ПЕРЕД созданием новой `__weak` функции, чтобы проверить — нет ли уже переопределения.

### Шаг 2.4b. НОВОЕ: `validate_project.go`

```go
package tools

import (
    "context"
    "fmt"
    "os"
    "path/filepath"
    "regexp"
    "strings"
)

type ValidateProjectTool struct{}

type ValidationResult struct {
    Valid       bool     `json:"valid"`
    Errors      []string `json:"errors"`
    Warnings    []string `json:"warnings"`
    CubeMXVer   string   `json:"cubemx_version"`
    FilesCount  int      `json:"files_count"`
}

func (t *ValidateProjectTool) Name() string { return "cubemx_validate_project" }
func (t *ValidateProjectTool) Description() string {
    return "Validate CubeMX project integrity BEFORE making changes. " +
           "Checks: USER CODE markers present, no duplicate weak overrides, " +
           "CubeMX version compatible. Call this FIRST."
}

func (t *ValidateProjectTool) Execute(ctx context.Context, projectRoot string) (any, error) {
    result := ValidationResult{Valid: true}
    
    // 1. Найти main.c, проверить USER CODE маркеры
    mainC := filepath.Join(projectRoot, "Core", "Src", "main.c")
    if _, err := os.Stat(mainC); os.IsNotExist(err) {
        result.Valid = false
        result.Errors = append(result.Errors, "main.c not found")
        return result, nil
    }
    
    content, _ := os.ReadFile(mainC)
    
    // Проверка: есть ли USER CODE BEGIN/END
    beginCount := strings.Count(string(content), "USER CODE BEGIN")
    endCount := strings.Count(string(content), "USER CODE END")
    
    if beginCount != endCount {
        result.Valid = false
        result.Errors = append(result.Errors, 
            fmt.Sprintf("Mismatched USER CODE markers: %d BEGIN vs %d END", beginCount, endCount))
    }
    
    if beginCount == 0 {
        result.Warnings = append(result.Warnings, 
            "No USER CODE blocks found. File may not be CubeMX-generated.")
    }
    
    // 2. Извлечь версию CubeMX из комментария
    verRe := regexp.MustCompile(`This file is part of STM32Cube.*?v(\d+\.\d+\.\d+)`)
    if matches := verRe.FindStringSubmatch(string(content)); len(matches) > 1 {
        result.CubeMXVer = matches[1]
    }
    
    // 3. Проверить дубликаты weak overrides
    weakTool := &FindWeakOverridesTool{}
    weakResult, _ := weakTool.Execute(ctx, projectRoot)
    if weakMap, ok := weakResult.(map[string]any); ok {
        overrides := weakMap["overrides"].([]WeakOverride)
        
        // Группировка по имени функции
        seen := make(map[string][]WeakOverride)
        for _, o := range overrides {
            seen[o.Function] = append(seen[o.Function], o)
        }
        
        for funcName, occurences := range seen {
            if len(occurences) > 1 {
                result.Valid = false
                result.Errors = append(result.Errors,
                    fmt.Sprintf("Duplicate override of %s in %d files", funcName, len(occurences)))
            }
        }
    }
    
    // 4. Подсчёт .c файлов
    filepath.Walk(projectRoot, func(path string, info os.FileInfo, err error) error {
        if strings.HasSuffix(path, ".c") {
            result.FilesCount++
        }
        return nil
    })
    
    return result, nil
}
```

**Использование:** Агент вызывает этот инструмент ПЕРВЫМ перед началом работы с проектом.

### Шаг 2.4c. НОВОЕ: Инструменты для J-Link/probe-rs

#### `flash_firmware.go` — прошивка через J-Link

```go
package tools

import (
    "context"
    "fmt"
    "os/exec"
    "path/filepath"
)

type FlashFirmwareTool struct{}

type FlashRequest struct {
    FirmwarePath string `json:"firmware_path"` // .elf or .bin
    Chip         string `json:"chip"`          // "STM32F407ZETx"
    Interface    string `json:"interface"`     // "SWD" (default) or "JTAG"
}

func (t *FlashFirmwareTool) Name() string { return "flash_firmware" }
func (t *FlashFirmwareTool) Description() string {
    return "Flash firmware to STM32 via J-Link. " +
           "Supports .elf and .bin files. Uses JFlash.exe CLI."
}

func (t *FlashFirmwareTool) Execute(ctx context.Context, req FlashRequest) (any, error) {
    // Валидация
    if req.Chip == "" { req.Chip = "STM32F407ZETx" }
    if req.Interface == "" { req.Interface = "SWD" }
    
    absPath, err := filepath.Abs(req.FirmwarePath)
    if err != nil { return nil, err }
    
    // Проверка существования файла
    if _, err := exec.LookPath("JFlash.exe"); err != nil {
        return nil, fmt.Errorf("JFlash.exe not found in PATH. Install Segger J-Link software")
    }
    
    // Прошивка через JFlash CLI
    cmd := exec.CommandContext(ctx, "JFlash.exe",
        "-device", req.Chip,
        "-if", req.Interface,
        "-auto", absPath)
    
    output, err := cmd.CombinedOutput()
    if err != nil {
        return nil, fmt.Errorf("flash failed: %w\nOutput: %s", err, string(output))
    }
    
    return map[string]any{
        "status":   "success",
        "firmware": absPath,
        "chip":     req.Chip,
        "output":   string(output),
    }, nil
}
```

**Альтернатива через probe-rs (для Rust проектов):**
```go
func (t *FlashFirmwareTool) Execute(ctx context.Context, req FlashRequest) (any, error) {
    // Для Rust проектов используем probe-rs
    cmd := exec.CommandContext(ctx, "probe-rs", "download",
        req.FirmwarePath,
        "--chip", req.Chip)
    
    output, err := cmd.CombinedOutput()
    // ...
}
```

#### `read_rtt_logs.go` — чтение RTT-логов

```go
package tools

import (
    "bufio"
    "context"
    "fmt"
    "os/exec"
    "strings"
    "time"
)

type ReadRTTLogsTool struct{}

type RTTRequest struct {
    Duration   int    `json:"duration"`     // seconds to read (default: 5)
    Channel    int    `json:"channel"`      // RTT channel (default: 0)
    Chip       string `json:"chip"`         // "STM32F407ZETx"
}

func (t *ReadRTTLogsTool) Name() string { return "read_rtt_logs" }
func (t *ReadRTTLogsTool) Description() string {
    return "Read RTT (Real-Time Transfer) logs from MCU via J-Link. " +
           "Used for defmt-rtt logging in Rust or printf-style debugging in C."
}

func (t *ReadRTTLogsTool) Execute(ctx context.Context, req RTTRequest) (any, error) {
    if req.Duration == 0 { req.Duration = 5 }
    if req.Chip == "" { req.Chip = "STM32F407ZETx" }
    
    // Запуск JLinkRTTClient
    cmd := exec.CommandContext(ctx, "JLinkRTTClient.exe",
        "-Device", req.Chip,
        "-If", "SWD",
        "-RTTChannel", fmt.Sprintf("%d", req.Channel))
    
    stdout, _ := cmd.StdoutPipe()
    if err := cmd.Start(); err != nil {
        return nil, fmt.Errorf("failed to start RTT client: %w", err)
    }
    
    // Чтение логов в течение указанного времени
    var logs []string
    done := make(chan struct{})
    
    go func() {
        scanner := bufio.NewScanner(stdout)
        for scanner.Scan() {
            logs = append(logs, scanner.Text())
        }
        close(done)
    }()
    
    time.Sleep(time.Duration(req.Duration) * time.Second)
    cmd.Process.Kill()
    <-done
    
    return map[string]any{
        "logs":     logs,
        "count":    len(logs),
        "duration": req.Duration,
        "channel":  req.Channel,
    }, nil
}
```

#### `debug_attach.go` — подключение GDB-отладчика

```go
package tools

import (
    "context"
    "fmt"
    "os/exec"
    "time"
)

type DebugAttachTool struct{}

type DebugRequest struct {
    Chip      string `json:"chip"`       // "STM32F407ZETx"
    Port      int    `json:"port"`       // GDB port (default: 2331)
    Interface string `json:"interface"`  // "SWD" or "JTAG"
}

func (t *DebugAttachTool) Name() string { return "debug_attach" }
func (t *DebugAttachTool) Description() string {
    return "Start J-Link GDB server for debugging. " +
           "Returns GDB port for IDE connection (VSCode, CLion, etc.)."
}

func (t *DebugAttachTool) Execute(ctx context.Context, req DebugRequest) (any, error) {
    if req.Port == 0 { req.Port = 2331 }
    if req.Chip == "" { req.Chip = "STM32F407ZETx" }
    if req.Interface == "" { req.Interface = "SWD" }
    
    // Проверка наличия GDB server
    if _, err := exec.LookPath("JLinkGDBServerCL.exe"); err != nil {
        return nil, fmt.Errorf("JLinkGDBServerCL.exe not found in PATH")
    }
    
    // Запуск GDB server в фоне
    cmd := exec.CommandContext(ctx, "JLinkGDBServerCL.exe",
        "-device", req.Chip,
        "-if", req.Interface,
        "-port", fmt.Sprintf("%d", req.Port),
        "-nogui",
        "-silent")
    
    if err := cmd.Start(); err != nil {
        return nil, fmt.Errorf("failed to start GDB server: %w", err)
    }
    
    // Дать серверу время на запуск
    time.Sleep(2 * time.Second)
    
    return map[string]any{
        "status":  "running",
        "port":    req.Port,
        "chip":    req.Chip,
        "command": fmt.Sprintf("target remote localhost:%d", req.Port),
        "pid":     cmd.Process.Pid,
    }, nil
}
```

**Использование:**
1. Агент вызывает `debug_attach` для запуска GDB server
2. Возвращает порт (например, 2331)
3. IDE (VSCode, CLion) подключается к `localhost:2331`
4. Агент может продолжать работу (прошивка, чтение логов)

### Шаг 2.5. Парсер CubeMX: `cubemx/parser.go`

```go
package cubemx

import (
    "os"
    "regexp"
)

type UserBlock struct {
    Name    string `json:"name"`
    Content string `json:"content"`
    Line    int    `json:"line"`
}

type ParseResult struct {
    UserBlocks     []UserBlock `json:"user_blocks"`
    WeakFunctions  []string    `json:"weak_functions"`
    GeneratedCode  string      `json:"-"` // не отдаём агенту
}

type Parser struct{}

func NewParser() *Parser { return &Parser{} }

func (p *Parser) Parse(filePath string) (*ParseResult, error) {
    data, err := os.ReadFile(filePath)
    if err != nil { return nil, err }
    content := string(data)

    result := &ParseResult{}

    // 1. Извлекаем USER CODE блоки (УЛУЧШЕННЫЙ regex)
    //    Поддерживает: /* USER CODE BEGIN 3 */, /*USER CODE BEGIN 3*/, 
    //                  /* USER CODE BEGIN 3 @brief */
    re := regexp.MustCompile(
        `(?s)/\*\s*(USER CODE BEGIN (\d+))(?:\s+@\w+)?\s*\*/(.*?)/\*\s*USER CODE END \2\s*\*/`,
    )
    for _, m := range re.FindAllStringSubmatch(content, -1) {
        result.UserBlocks = append(result.UserBlocks, UserBlock{
            Name:    m[1], // "USER CODE BEGIN 3"
            Content: m[3], // содержимое
        })
    }

    // 2. Находим переопределённые __weak функции
    //     Ищем: void FunctionName(...) { (без __weak в сигнатуре)
    weakRe := regexp.MustCompile(`(?m)^void\s+(\w+)\s*\([^)]*\)\s*\{`)
    for _, m := range weakRe.FindAllStringSubmatch(content, -1) {
        funcName := m[1]
        // Проверяем, не является ли это локальной функцией
        if isKnownHalMsp(funcName) {
            result.WeakFunctions = append(result.WeakFunctions, funcName)
        }
    }

    return result, nil
}

func isKnownHalMsp(name string) bool {
    // Типичные HAL Msp функции (переопределяемые)
    halFuncs := []string{
        "HAL_GPIO_MspInit", "HAL_GPIO_MspDeInit",
        "HAL_TIM_Base_MspInit", "HAL_TIM_Base_MspDeInit",
        "HAL_UART_MspInit", "HAL_UART_MspDeInit",
        "HAL_SPI_MspInit", "HAL_SPI_MspDeInit",
        "HAL_I2C_MspInit", "HAL_I2C_MspDeInit",
        "HAL_ADC_MspInit", "HAL_ADC_MspDeInit",
        "HAL_CAN_MspInit", "HAL_CAN_MspDeInit",
    }
    for _, f := range halFuncs {
        if f == name { return true }
    }
    return false
}
```

**Улучшения:**
- Regex извлекает номер блока (`\d+`) и использует его для поиска END
- Поддерживает опциональные атрибуты (`@brief`)
- Находит переопределённые HAL Msp функции

### Шаг 2.6. RAG-поиск: `rag/searcher.go`

```go
package rag

import (
    "context"
    "github.com/qdrant/go-client/qdrant"
)

type SearchResult struct {
    Text       string  `json:"text"`
    Peripheral string  `json:"peripheral"`
    Register   string  `json:"register"`
    Score      float32 `json:"score"`
    Source     string  `json:"source"`
}

type HybridSearcher struct {
    qdrant   *QdrantClient
    embedder *OllamaEmbedder
    collection string
}

func (s *HybridSearcher) Search(ctx context.Context, query string, filters map[string]string) ([]SearchResult, error) {
    // 1. Эмбеддинг запроса (с retry)
    vector, err := s.embedder.Embed(ctx, query)
    if err != nil { return nil, err }

    // 2. Hybrid search: vector + BM25
    req := &qdrant.SearchPoints{
        CollectionName: s.collection,
        Vector:         vector,
        Limit:          5,
        WithPayload:    qdrant.NewWithPayload(true),
    }

    // 3. Добавляем фильтры (mcu, peripheral)
    if filters["peripheral"] != "" {
        req.Filter = &qdrant.Filter{
            Must: []*qdrant.Condition{
                {
                    ConditionOneOf: &qdrant.Condition_Field{
                        Field: &qdrant.FieldCondition{
                            Key: "peripheral",
                            Match: &qdrant.Match{
                                MatchValue: &qdrant.Match_Keyword{Keyword: filters["peripheral"]},
                            },
                        },
                    },
                },
            },
        }
    }

    // 4. Выполняем запрос
    resp, err := s.qdrant.Search(ctx, req)
    if err != nil { return nil, err }

    // 5. Маппинг в SearchResult
    var results []SearchResult
    for _, hit := range resp.Result {
        results = append(results, SearchResult{
            Text:       hit.Payload["text"].GetStringValue(),
            Peripheral: hit.Payload["peripheral"].GetStringValue(),
            Register:   hit.Payload["register"].GetStringValue(),
            Score:      hit.Score,
            Source:     hit.Payload["source"].GetStringValue(),
        })
    }

    return results, nil
}
```

### Шаг 2.6a. НОВОЕ: `rag/embedder.go` с retry и кэшированием

```go
package rag

import (
    "bytes"
    "context"
    "encoding/json"
    "fmt"
    "net/http"
    "sync"
    "time"
)

type OllamaEmbedder struct {
    url      string
    model    string
    cache    map[string][]float32
    cacheMux sync.RWMutex
    client   *http.Client
}

func NewOllamaEmbedder(url, model string) *OllamaEmbedder {
    return &OllamaEmbedder{
        url:    url,
        model:  model,
        cache:  make(map[string][]float32),
        client: &http.Client{Timeout: 30 * time.Second},
    }
}

func (e *OllamaEmbedder) Embed(ctx context.Context, text string) ([]float32, error) {
    // 1. Проверка кэша
    e.cacheMux.RLock()
    if cached, ok := e.cache[text]; ok {
        e.cacheMux.RUnlock()
        return cached, nil
    }
    e.cacheMux.RUnlock()

    // 2. Запрос к Ollama с retry (3 попытки)
    var vector []float32
    var err error
    
    for attempt := 0; attempt < 3; attempt++ {
        vector, err = e.embed(text)
        if err == nil {
            break
        }
        if attempt < 2 {
            time.Sleep(time.Duration(attempt+1) * time.Second)
        }
    }
    
    if err != nil {
        return nil, fmt.Errorf("ollama unavailable after 3 attempts: %w", err)
    }

    // 3. Сохранение в кэш
    e.cacheMux.Lock()
    e.cache[text] = vector
    e.cacheMux.Unlock()

    return vector, nil
}

func (e *OllamaEmbedder) embed(text string) ([]float32, error) {
    payload := map[string]any{
        "model": e.model,
        "input": []string{text},
    }
    
    body, _ := json.Marshal(payload)
    resp, err := e.client.Post(e.url+"/api/embed", "application/json", bytes.NewReader(body))
    if err != nil {
        return nil, err
    }
    defer resp.Body.Close()

    if resp.StatusCode != 200 {
        return nil, fmt.Errorf("ollama returned %d", resp.StatusCode)
    }

    var result struct {
        Embeddings [][]float32 `json:"embeddings"`
    }
    if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
        return nil, err
    }
    
    if len(result.Embeddings) == 0 {
        return nil, fmt.Errorf("empty embedding returned")
    }

    return result.Embeddings[0], nil
}

// ClearCache очищает кэш (вызывать при смене модели)
func (e *OllamaEmbedder) ClearCache() {
    e.cacheMux.Lock()
    e.cache = make(map[string][]float32)
    e.cacheMux.Unlock()
}
```

**Преимущества:**
- **Retry**: 3 попытки с exponential backoff (1s, 2s, 3s)
- **Кэш**: Повторные запросы того же текста мгновенны
- **Thread-safe**: `sync.RWMutex` для конкурентного доступа
- **Таймаут**: 30 секунд на запрос

### Шаг 2.7. `Dockerfile` для MCP

```dockerfile
FROM golang:1.22-alpine AS builder
WORKDIR /app
COPY go.mod go.sum ./
RUN go mod download
COPY . .
RUN CGO_ENABLED=0 go build -o /mcp-server .

FROM alpine:3.19
RUN apk add --no-cache ca-certificates
COPY --from=builder /mcp-server /mcp-server
ENTRYPOINT ["/mcp-server"]
```

**Чек-лист Фазы 2:**
- [ ] Создана структура проекта (с новыми файлами для J-Link)
- [ ] Реализован STDIO-транспорт (основной для Cursor/Cline)
- [ ] Реализован `cubemx_insert_code` с улучшенным regex (поддержка всех форматов)
- [ ] Реализован `cubemx_read_user_blocks` (возвращает ТОЛЬКО user code)
- [ ] Реализован `cubemx_find_weak_overrides` (предотвращает дубликаты)
- [ ] Реализован `cubemx_validate_project` (валидация перед работой)
- [ ] Реализован `query_hardware_docs` с hybrid search
- [ ] Реализован `check_f407_rules` (CCM+DMA, NVIC и т.д.)
- [ ] Реализован `embedder.go` с retry + кэш
- [ ] **НОВОЕ:** Реализован `flash_firmware` (прошивка через JFlash.exe)
- [ ] **НОВОЕ:** Реализован `read_rtt_logs` (чтение RTT-логов)
- [ ] **НОВОЕ:** Реализован `debug_attach` (подключение GDB server)
- [ ] Написаны unit-тесты для regex (3+ формата CubeMX)
- [ ] Скомпилирован `mcp-server.exe` для Windows

---

## 🎯 ФАЗА 3: Интеграция с агентом (1 день)

### Шаг 3.1. Конфигурация для Cursor / Cline

**Изменение:** MCP Server запускается **нативно**, не через `docker exec`.

**Для Cursor** (`.cursor/mcp.json`):
```json
{
  "mcpServers": {
    "stm32-f407": {
      "command": "C:\\Users\\mdari\\projects\\mcp\\halo-f407-mcp\\mcp-server\\mcp-server.exe",
      "args": [],
      "env": {
        "QDRANT_URL": "http://localhost:6334",
        "OLLAMA_URL": "http://localhost:11434",
        "PROJECT_ROOT": "C:\\Users\\mdari\\projects\\stm32\\workspace"
      }
    }
  }
}
```

**Для Cline** (VS Code):
```json
{
  "mcpServers": {
    "stm32-f407": {
      "type": "stdio",
      "command": "C:\\Users\\mdari\\projects\\mcp\\halo-f407-mcp\\mcp-server\\mcp-server.exe",
      "args": [],
      "env": {
        "QDRANT_URL": "http://localhost:6334",
        "OLLAMA_URL": "http://localhost:11434"
      }
    }
  }
}
```

**Альтернатива (для разработки):** Использовать `go run` вместо скомпилированного binary:
```json
{
  "mcpServers": {
    "stm32-f407": {
      "command": "go",
      "args": ["run", "."],
      "cwd": "C:\\Users\\mdari\\projects\\mcp\\halo-f407-mcp\\mcp-server"
    }
  }
}
```

**Преимущества нативного запуска:**
- Прямой доступ к `localhost:11434` (Ollama) без `host.docker.internal`
- Прямой доступ к J-Link CLI (`JFlash.exe`, `JLinkRTTClient.exe`)
- Прямой доступ к USB-устройствам (probe-rs, ST-Link)
- Упрощённая отладка (можно запустить `dlv debug` для Go)

### Шаг 3.2. Системный промпт для агента

Создайте `.cursorrules` или `CLAUDE.md`:

```markdown
# STM32F407 Agent Rules

## Роль
Ты — embedded-инженер, специализирующийся на STM32F407.
Твоя главная задача — НЕ СЛОМАТЬ код, сгенерированный CubeMX.

## Порядок работы с инструментами

### ПЕРЕД началом работы (ОБЯЗАТЕЛЬНО)
1. Вызови `cubemx_validate_project` — проверь целостность проекта
2. Вызови `cubemx_find_weak_overrides` — узнай, какие функции уже переопределены
3. Вызови `cubemx_read_user_blocks` — прочитай существующий user code

### Во время работы
- Для вставки кода используй ТОЛЬКО `cubemx_insert_code`
- Для вопросов о регистрах — `query_hardware_docs`
- Для проверки аппаратных ограничений — `check_f407_rules`

### ПЕРЕД созданием __weak функции
- Проверь через `cubemx_find_weak_overrides`, нет ли уже переопределения
- Если есть — используй существующее, не создавай дубликат

## Жёсткие правила

### 1. Работа с файлами CubeMX
- НИКОГДА не пиши код вне блоков `/* USER CODE BEGIN X */ ... /* USER CODE END X */`
- Используй ТОЛЬКО инструмент `cubemx_insert_code`
- Перед правкой ВСЕГДА вызывай `cubemx_read_user_blocks` чтобы понять контекст
- Для переопределения HAL-поведения создавай `__weak` функции ВНЕ блоков USER CODE,
  но в конце файла main.c (или в отдельном файле)

### 2. Порядок инициализации
- USER CODE BEGIN 1: includes, defines, приватные переменные
- USER CODE BEGIN 2: прототипы функций
- USER CODE BEGIN 3: вызовы пользовательской инициализации (после MX_*_Init())
- USER CODE BEGIN 4: код в while(1)

### 3. Аппаратные ограничения STM32F407
- DMA-буферы НЕЛЬЗЯ размещать в CCM RAM (`.ccmram`) — только в SRAM1/SRAM2
- DMA Stream/Channel маппинг жёсткий (см. RM0090 ch.9)
- NVIC Priority Grouping обычно 4 (4 бита preemption, 0 sub)
- USB/SDIO требуют ровно 48 МГц
- Таймеры APB1/APB2 умножают частоту на x2/x4 если prescaler != 1

### 4. Документация
- Для вопросов о регистрах ВСЕГДА используй `query_hardware_docs`
- При сомнениях — проверяй `check_f407_rules`

### 5. Стиль кода
- Используй HAL-функции где возможно
- Прямой доступ к регистрам — только через `PERIPH->REG` (CMSIS)
- Комментарии на русском или английском, кратко
```

**Изменения:**
- Добавлен явный порядок вызова инструментов (validate → find_weak → read → insert)
- Подчёркнута важность проверки дубликатов `__weak`
- Уточнены сценарии использования новых инструментов

**Чек-лист Фазы 3:**
- [ ] MCP-сервер подключён к Cursor/Cline
- [ ] Инструменты видны в интерфейсе агента (6 шт.)
- [ ] Системный промпт загружен (с новым порядком вызова)
- [ ] Тестовый вызов `cubemx_validate_project` работает
- [ ] Тестовый вызов `cubemx_find_weak_overrides` работает
- [ ] Тестовый вызов `cubemx_read_user_blocks` работает

---

## 🎯 ФАЗА 4: Тестирование и валидация (3-4 дня)

### Шаг 4.1. Тест-кейсы

Создайте тестовый проект CubeMX для STM32F407 и прогоните сценарии:

**CubeMX tools (8 тестов):**

| № | Сценарий | Ожидаемое поведение | Инструменты |
|---|----------|---------------------|-------------|
| 1 | "Настрой TIM1 для ШИМ 1kHz" | Агент использует HAL, правит USER CODE BEGIN 3 | validate → read → insert |
| 2 | "Добавь DMA для UART1 RX" | Агент проверяет Stream/Channel маппинг через `query_hardware_docs` | query_docs → insert |
| 3 | "Положи буфер DMA в CCM RAM" | Агент получает отказ от `check_f407_rules` | check_rules → (отказ) |
| 4 | "Настрой dead-time для complementary PWM" | Агент читает `TIM1_BDTR` через RAG | query_docs → insert |
| 5 | "Попробуй изменить код вне USER CODE" | MCP возвращает ошибку | insert → (ошибка) |
| 6 | "Создай override для HAL_GPIO_MspInit" | Агент проверяет дубликаты через `find_weak_overrides` | find_weak → insert |
| 7 | "Работай с проектом, где уже есть 3 override" | Агент видит существующие, не создаёт дубликат | validate → find_weak → read |
| 8 | "Проект CubeMX 6.0 vs 6.12 (разные форматы)" | Regex корректно парсит оба формата | (автоматические тесты) |

**J-Link/hardware tools (3 теста):**

| № | Сценарий | Ожидаемое поведение | Инструменты |
|---|----------|---------------------|-------------|
| 9 | "Прошей firmware.bin через J-Link" | Агент вызывает `flash_firmware`, возвращает статус | flash_firmware → (success) |
| 10 | "Прочитай RTT-логи за последние 5 секунд" | Агент запускает `read_rtt_logs`, возвращает логи | read_rtt_logs → (logs) |
| 11 | "Подключи GDB-отладчик к порту 2331" | Агент вызывает `debug_attach`, возвращает порт | debug_attach → (port: 2331) |

**Итого: 11 тест-кейсов** (добавлены 3 для J-Link).

### Шаг 4.2. Метрики качества

Отслеживайте:
- **Точность RAG:** % запросов, где топ-1 результат релевантен (цель >80%)
- **Безопасность:** количество попыток записи вне USER CODE (цель = 0)
- **Дубликаты weak:** количество попыток создать дубликат override (цель = 0 после внедрения `find_weak_overrides`)
- **Токены на запрос:** средний расход (цель <4000 tokens/query)
- **Время отклика:** p95 latency MCP-инструментов (цель <500ms)
- **Кэш embedder:** % hit rate (цель >30% для повторяющихся запросов)
- **Валидация проекта:** % проектов, прошедших `validate_project` с первого раза (цель >90%)

### Шаг 4.3. Итерации

После первых тестов:
1. Если RAG даёт нерелевантные результаты → улучшайте чанкинг, добавляйте больше тегов
2. Если `nomic-embed-text` плохо ищет регистры → пробуйте `intfloat/multilingual-e5-large`
3. Если агент всё равно ломает CubeMX → ужесточайте regex в `patcher.go`
4. Если не хватает знаний → добавляйте разделы в `f407_known_issues.md`

**Чек-лист Фазы 4:**
- [ ] Прогнаны все 5 тест-кейсов
- [ ] Собраны метрики
- [ ] Внесены правки по результатам тестов
- [ ] Документация обновлена

---

## 📊 Итоговая дорожная карта

| Фаза | Исходный срок | Скорректированный срок | Результат |
|------|---------------|------------------------|-----------|
| **0. Подготовка docs** | 1-2 дня | **4-6 дней** | MD-база RM0090 + Datasheet + Errata (после Marker) |
| **1. Инфраструктура** | 0.5 дня | 0.5 дня | Docker Compose с Qdrant (только Qdrant) |
| **2. MCP на Go** | 3-5 дней | **7-10 дней** | **9 инструментов** (добавлены 3 для J-Link), retry+кэш |
| **3. Интеграция** | 1 день | 1 день | Подключение к Cursor/Cline, системный промпт |
| **4. Тестирование** | 2-3 дня | **3-4 дня** | 11 тест-кейсов (добавлены 3 для J-Link), метрики |
| **ИТОГО** | ~2 недели | **~3-4 недели** | Рабочий хирург-агент для F407 + hardware integration |

**Ключевые изменения (итого):**
- ✅ Парсинг PDF через Marker (2-3 дня)
- ✅ Добавлены 5 новых инструментов: `find_weak_overrides`, `validate_project`, `flash_firmware`, `read_rtt_logs`, `debug_attach`
- ✅ Улучшен regex для USER CODE (поддержка всех форматов CubeMX)
- ✅ Добавлен retry + кэш для Ollama embedder
- ✅ **MCP Server работает нативно на Windows** (не в Docker)
- ✅ Прямая интеграция с J-Link/probe-rs (прошивка, RTT-логи, отладка)
- ✅ Кросс-платформенный Docker (только для Qdrant)
- ✅ Добавлены 6 новых тест-кейсов (итого 11)

---

## ⚠️ Критические точки внимания

1. **Не начинайте с MCP — начните с docs.** Качество RAG на 90% определяется качеством MD-чанков, а не кодом.
2. **Marker для PDF — не идеален.** После конвертации RM0090 вручную проверьте 5-10 случайных страниц. Таблицы регистров могут потребовать ручной чистки.
3. **Regex для USER CODE — ваш щит.** Тестируйте его на разных версиях CubeMX (6.0, 6.12, 7.0) — они могут менять формат комментариев.
4. **Errata Sheet — золото.** Большинство "магических" багов F407 описаны именно там. Без неё агент будет наступать на грабли.
5. **Локальный Ollama vs облачный.** Для `nomic-embed-text` локальный вариант оптимален — модель маленькая (~270MB), инференс быстрый.
6. **Версионирование базы знаний.** Храните MD-файлы в git. При обновлении RM0090 от ST — пересобирайте коллекцию Qdrant с `schema_version: 2`.
7. **Кэш embedder — экономия токенов.** При повторных запросах того же текста (а они будут) кэш ускоряет работу в 10 раз.
8. **find_weak_overrides — критичен.** Без этого инструмента агент ОБЯЗАТЕЛЬНО создаст дубликаты `__weak` функций. Это не "nice to have", а must-have.
9. **Docker на Windows.** `host.docker.internal` работает автоматически. Не раскомментируйте `extra_hosts` на Windows/Mac.
10. **validate_project — первый вызов.** Агент должен вызывать этот инструмент ПЕРВЫМ перед началом работы. Иначе рискуете работать с повреждённым проектом.

## 🎯 Приоритеты реализации

**Критичный путь (без этого не работает):**
1. Парсинг PDF → MD (Marker)
2. Базовые инструменты: `read_user_blocks`, `insert_code`
3. RAG-поиск: `query_hardware_docs`
4. Проверка правил: `check_f407_rules`

**Важно (резко повышает качество):**
5. `find_weak_overrides` — предотвращает дубликаты
6. `validate_project` — валидация перед работой
7. Retry + кэш для embedder — отказоустойчивость

**Nice to have (можно отложить):**
8. SSE-транспорт (если не нужен удалённый доступ)
9. Расширенный список HAL-функций в `find_weak_overrides`
10. Оптимизация чанкинга (после первых тестов)

Готов помочь с конкретным файлом (например, написать полный `parser.go` с обработкой edge cases или скрипт парсинга PDF→MD). С чего начнём?
