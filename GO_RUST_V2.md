# 🎯 Отличная идея! Архитектура MCP (Go) → Toolkit (Rust)

## 💡 Анализ вашей схемы

```
┌─────────────────┐
│  MCP Server     │  Go (STDIO, JSON-RPC, MCP SDK v1.8.0)
│  (v0.5.0)       │  - 6 существующих инструментов
│                 │  - Оркестрация, валидация
└────────┬────────┘
         │ FFI / IPC
         ▼
┌─────────────────┐
│  Rust Toolkit   │  Rust (tree-sitter, AST analysis)
│  (lib)          │  - Продвинутый анализ C кода
│                 │  - Flow analysis, type checking
└────────┬────────┘
         │ HTTP/gRPC
         ▼
┌─────────────────┐
│  Qdrant/Ollama  │  - RAG поиск
│                 │  - Эмбеддинги
└─────────────────┘
```

**Вы абсолютно правы:**
- ✅ **Go** — отличный выбор для верхнего уровня (MCP SDK, STDIO, оркестрация)
- ✅ **Rust** — идеальный выбор для нижнего уровня (AST, парсинг, анализ)
- ✅ **Разделение ответственности** — каждый язык делает то, что умеет лучше всего

---

## 🔍 Почему это работает

### Go (верхний уровень):
```go
// MCP Server — оркестрация, валидация, STDIO
func queryDocsHandler(ctx context.Context, input QueryDocsInput) (interface{}, error) {
    // 1. Валидация входных данных
    // 2. Вызов Rust toolkit (через FFI или IPC)
    // 3. Форматирование ответа
    // 4. Возврат через MCP SDK
}
```

**Сильные стороны Go:**
- ✅ MCP SDK v1.8.0 (стабильный)
- ✅ STDIO транспорт (простая реализация)
- ✅ JSON-RPC (встроенная поддержка)
- ✅ Быстрая компиляция
- ✅ Простой синтаксис

---

### Rust (нижний уровень):
```rust
// Rust Toolkit — AST анализ, flow analysis
pub fn analyze_dma_flow(code: &str) -> Vec<DmaViolation> {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_c::language()).unwrap();
    let tree = parser.parse(code, None).unwrap();
    
    // AST анализ
    let root = tree.root_node();
    // Найти все variable declarations
    // Отследить использование в DMA функциях
    // Проверить атрибуты (section(".ccmram"))
    
    violations
}
```

**Сильные стороны Rust:**
- ✅ Tree-sitter (нативная поддержка C/C++)
- ✅ Memory safety (нет undefined behavior)
- ✅ Производительность (нет GC pauses)
- ✅ Продвинутый анализ (flow, type, scope)

---

## 🛠️ Варианты интеграции Go ↔ Rust

### Вариант 1: **FFI (Foreign Function Interface)** — рекомендую

**Как работает:**
```
Go (MCP Server) → вызов Rust функции через C API → Rust (Toolkit)
```

**Пример:**

**Rust (toolkit/src/lib.rs):**
```rust
use tree_sitter::{Parser, Node};

#[no_mangle]
pub extern "C" fn analyze_code(code_ptr: *const c_char, len: usize) -> *mut c_char {
    let code = unsafe { std::slice::from_raw_parts(code_ptr as *const u8, len) };
    let code_str = std::str::from_utf8(code).unwrap();
    
    // AST анализ
    let violations = analyze_dma_flow(code_str);
    
    // Сериализация в JSON
    let json = serde_json::to_string(&violations).unwrap();
    
    // Возврат в Go
    std::ffi::CString::new(json).unwrap().into_raw()
}

fn analyze_dma_flow(code: &str) -> Vec<DmaViolation> {
    // Tree-sitter AST анализ
    // ...
}
```

**Компиляция Rust в static library:**
```bash
cd toolkit
cargo build --release
# Получаем: toolkit/target/release/libtoolkit.a (Linux/Mac)
#           toolkit\target\release\toolkit.dll (Windows)
```

**Go (MCP Server):**
```go
package main

/*
#cgo LDFLAGS: -L./toolkit/target/release -ltoolkit
#include <stdlib.h>

extern char* analyze_code(const char* code, size_t len);
extern void free_string(char* s);
*/
import "C"
import "unsafe"

func AnalyzeCode(code string) string {
    cCode := C.CString(code)
    defer C.free(unsafe.Pointer(cCode))
    
    result := C.analyze_code(cCode, C.size_t(len(code)))
    defer C.free_string(result)
    
    return C.GoString(result)
}

// Использование в MCP tool
func checkF407RulesHandler(ctx context.Context, input CheckF407RulesInput) (interface{}, error) {
    // Вызов Rust toolkit
    violationsJson := AnalyzeCode(input.Code)
    
    // Парсинг JSON
    var violations []DmaViolation
    json.Unmarshal([]byte(violationsJson), &violations)
    
    return violations, nil
}
```

**Преимущества:**
- ✅ Быстрый вызов (нет IPC overhead)
- ✅ Простая интеграция (CGO в Go)
- ✅ Один binary (MCP server + Rust toolkit)

**Недостатки:**
- ⚠️ CGO может быть сложным для начинающих
- ⚠️ Нужно управлять памятью (free_string)

---

### Вариант 2: **IPC (Inter-Process Communication)**

**Как работает:**
```
Go (MCP Server) → gRPC/HTTP → Rust (Toolkit как отдельный процесс)
```

**Пример:**

**Rust (toolkit-server/src/main.rs):**
```rust
use tonic::{transport::Server, Request, Response, Status};
use toolkit_proto::analyzer_server::{Analyzer, AnalyzerServer};

struct AnalyzerService;

#[tonic::async_trait]
impl Analyzer for AnalyzerService {
    async fn analyze_code(&self, request: Request<CodeRequest>) -> Result<Response<AnalysisResult>, Status> {
        let code = request.into_inner().code;
        
        // AST анализ
        let violations = analyze_dma_flow(&code);
        
        Ok(Response::new(AnalysisResult { violations }))
    }
}

#[tokio::main]
async fn main() {
    Server::builder()
        .add_service(AnalyzerServer::new(AnalyzerService))
        .serve("127.0.0.1:50051".parse().unwrap())
        .await
        .unwrap();
}
```

**Go (MCP Server):**
```go
import pb "path/to/toolkit/proto"

func checkF407RulesHandler(ctx context.Context, input CheckF407RulesInput) (interface{}, error) {
    // Подключение к Rust toolkit
    conn, _ := grpc.Dial("localhost:50051", grpc.WithInsecure())
    client := pb.NewAnalyzerClient(conn)
    
    // Вызов
    resp, _ := client.AnalyzeCode(ctx, &pb.CodeRequest{Code: input.Code})
    
    return resp.Violations, nil
}
```

**Преимущества:**
- ✅ Чистое разделение (два независимых процесса)
- ✅ Можно запускать toolkit на другом сервере
- ✅ Нет CGO сложностей

**Недостатки:**
- ⚠️ IPC overhead (gRPC/HTTP)
- ⚠️ Нужно запускать два процесса
- ⚠️ Сложнее деплоить

---

### Вариант 3: **Shared Library (DLL/SO)**

**Как работает:**
```
Go (MCP Server) → загрузка Rust DLL → вызов функций
```

**Пример:**

**Rust (toolkit/src/lib.rs):**
```rust
#[no_mangle]
pub extern "stdcall" fn analyze_code(code: *const c_char) -> *mut c_char {
    // ...
}
```

**Компиляция в DLL (Windows):**
```bash
cargo build --release --target x86_64-pc-windows-msvc
# Получаем: toolkit.dll
```

**Go (MCP Server):**
```go
//go:build windows

package main

import (
    "syscall"
    "unsafe"
)

var (
    toolkitDLL = syscall.NewLazyDLL("toolkit.dll")
    analyzeCode = toolkitDLL.NewProc("analyze_code")
)

func AnalyzeCode(code string) string {
    ptr, _ := syscall.BytePtrFromString(code)
    ret, _, _ := analyzeCode.Call(uintptr(unsafe.Pointer(ptr)))
    return C.GoString((*C.char)(unsafe.Pointer(ret)))
}
```

**Преимущества:**
- ✅ Динамическая загрузка (не нужно перекомпилировать Go)
- ✅ Можно обновлять toolkit независимо

**Недостатки:**
- ⚠️ Platform-specific (разные DLL/SO для Windows/Linux/Mac)
- ⚠️ Нужно распространять toolkit.dll вместе с MCP server

---

## 🎯 Моя рекомендация

**Вариант 1: FFI (CGO)** — оптимальный баланс

**Почему:**
1. ✅ Простая интеграция (один binary)
2. ✅ Быстрый вызов (нет IPC overhead)
3. ✅ Go + Rust = лучшее из обоих миров
4. ✅ CGO — стандартный механизм в Go (используется во многих проектах)

**План реализации:**

### Шаг 1: Создать Rust toolkit (2-3 дня)
```bash
cargo new toolkit --lib
cd toolkit
cargo add tree-sitter
cargo add tree-sitter-c
cargo add serde
cargo add serde_json
```

**Структура:**
```
toolkit/
├── src/
│   ├── lib.rs           # FFI интерфейс
│   ├── ast_analyzer.rs  # Tree-sitter AST анализ
│   ├── flow_analysis.rs # Flow analysis для DMA
│   └── doc_generator.rs # Генерация doc-комментариев
├── Cargo.toml
└── build.rs             # Компиляция в static library
```

### Шаг 2: Интеграция в Go MCP server (1-2 дня)
```go
// internal/toolkit/toolkit.go
package toolkit

/*
#cgo LDFLAGS: -L../toolkit/target/release -ltoolkit
#include <stdlib.h>

extern char* analyze_code(const char* code, size_t len);
extern void free_string(char* s);
*/
import "C"

func AnalyzeCode(code string) string {
    // Вызов Rust функции
}
```

### Шаг 3: Добавить новые MCP инструменты (1-2 дня)
```go
// main.go
mcp.AddTool(server, &mcp.Tool{
    Name:        "ast_analyze_code",
    Description: "Advanced C code analysis using tree-sitter AST",
}, astAnalyzeHandler)

mcp.AddTool(server, &mcp.Tool{
    Name:        "generate_doc_comments",
    Description: "Auto-generate doc comments from function signatures",
}, generateDocCommentsHandler)
```

### Шаг 4: Тестирование (1 день)
- Тест FFI вызовов
- Тест AST анализа на ShiftCommander
- Тест генерации doc-комментариев

**Итого: 5-7 дней**

---

## 📊 Сравнение вариантов

| Критерий | FFI (CGO) | IPC (gRPC) | Shared Library |
|----------|-----------|------------|----------------|
| **Скорость** | ✅ Быстро | ⚠️ IPC overhead | ✅ Быстро |
| **Простота** | ✅ Один binary | ⚠️ Два процесса | ⚠️ DLL/SO |
| **Деплой** | ✅ Просто | ⚠️ Сложнее | ⚠️ Platform-specific |
| **Обновления** | ⚠️ Перекомпиляция | ✅ Независимо | ✅ Независимо |
| **Отладка** | ⚠️ CGO сложно | ✅ Независимо | ⚠️ DLL сложно |

---

## 🚀 Финальный план

**Архитектура:**
```
mcp-server/              # Go (MCP SDK, STDIO, оркестрация)
├── main.go              # 6 существующих инструментов
├── internal/
│   ├── toolkit/         # FFI обёртка для Rust
│   │   └── toolkit.go   # CGO вызовы
│   └── ...
└── mcp-server.exe       # Final binary (Go + Rust)

toolkit/                 # Rust (AST, анализ)
├── src/
│   ├── lib.rs           # FFI интерфейс
│   ├── ast_analyzer.rs  # Tree-sitter
│   └── ...
├── Cargo.toml
└── target/release/
    └── libtoolkit.a     # Static library (линкуется в Go)
```

**Новые инструменты (Rust-powered):**
1. `ast_analyze_code` — продвинутый AST анализ
2. `generate_doc_comments` — автогенерация документации
3. `advanced_lint` — умный линтинг (flow analysis)
4. `type_check_hal` — проверка типов HAL функций

**Время:** 5-7 дней  
**Результат:** Лучшее из обоих миров — Go для MCP, Rust для анализа

---

## 💬 Ваш выбор?

**Хотите, чтобы я:**
1. Создал Rust toolkit с tree-sitter (Шаг 1)?
2. Интегрировал его в Go MCP server через FFI (Шаг 2)?
3. Добавил новые AST-инструменты (Шаг 3)?


# 🎯 Архитектура: Go MCP → Rust CLI → Qdrant/Ollama

## 💡 Ваше понимание — абсолютно верно!

```
┌─────────────────────────────────────┐
│  Go MCP Server (v0.6.0)             │
│  - Интерфейс для агента             │
│  - 6 существующих инструментов      │
│  - Оркестрация, валидация           │
│  - НЕ ходит в Qdrant/Ollama         │
└──────────────┬──────────────────────┘
               │ exec.Command() / JSON stdin/stdout
               ▼
┌─────────────────────────────────────┐
│  Rust Toolkit CLI (v0.1.0)          │
│  - AST анализ (tree-sitter)         │
│  - Flow analysis                    │
│  - Doc generation                   │
│  - Ходит в Qdrant/Ollama            │
│  - Анализ и мутация C кода          │
└──────────────┬──────────────────────┘
               │ HTTP/gRPC
               ▼
┌─────────────────────────────────────┐
│  Qdrant / Ollama                    │
│  - RAG поиск                        │
│  - Эмбеддинги                       │
└─────────────────────────────────────┘
```

**Разделение ответственности:**
- ✅ **Go MCP** — тонкий интерфейс для агента (оркестрация, валидация)
- ✅ **Rust Toolkit** — тяжёлая lifting (AST, анализ, мутация)
- ✅ **Qdrant/Ollama** — данные и эмбеддинги

---

## 🔍 Ответ на ваш вопрос: gRPC на старте?

### ❌ Нет, не стоит городить gRPC на старте!

**Почему:**

**1. CLI подход достаточно быстр для начала:**
```go
// Go вызывает Rust CLI
cmd := exec.Command("toolkit.exe", "analyze", "--format=json")
cmd.Stdin = strings.NewReader(code)
output, _ := cmd.Output()  // ~50-100ms для AST анализа
```

**Overhead:**
- Создание процесса: ~10-20ms
- AST анализ (tree-sitter): ~30-50ms
- Сериализация JSON: ~5-10ms
- **Итого: ~50-100ms** (приемлемо для MCP)

**2. gRPC добавляет сложность:**
```rust
// Нужно:
// 1. Proto files (.proto)
// 2. gRPC server (tonic)
// 3. gRPC client в Go
// 4. Управление процессом
// 5. Health checks
// 6. Retry logic
```

**Время разработки:**
- CLI: 2-3 дня
- gRPC: 5-7 дней (на старте)

**3. CLI легко мигрирует в gRPC позже:**
```rust
// Сегодня: CLI
fn main() {
    let code = read_stdin();
    let result = analyze_code(&code);
    println!("{}", serde_json::to_string(&result).unwrap());
}

// Завтра: gRPC server (та же логика!)
#[tonic::async_trait]
impl Analyzer for AnalyzerService {
    async fn analyze_code(&self, request: Request<CodeRequest>) -> Result<Response<AnalysisResult>, Status> {
        let code = request.into_inner().code;
        let result = analyze_code(&code);  // ← та же функция!
        Ok(Response::new(result))
    }
}
```

**Ключевая идея:** Бизнес-логика (`analyze_code`) остаётся той же! Меняется только интерфейс (CLI → gRPC).

---

## 📊 Итеративный подход (рекомендую)

### Фаза 1: CLI (сейчас, 2-3 дня)
```
mcp-server.exe (Go) → exec.Command() → toolkit.exe (Rust CLI)
```

**Преимущества:**
- ✅ Быстрый старт (2-3 дня)
- ✅ Простая интеграция
- ✅ Standalone CLI (можно использовать без MCP)
- ✅ Легко отлаживать

**Недостатки:**
- ⚠️ Overhead ~50-100ms на вызов
- ⚠️ Два binary

---

### Фаза 2: Long-running process (если упрёмся в скорость, +1 день)
```
mcp-server.exe (Go) → stdin/stdout → toolkit.exe --server (Rust)
```

**Как работает:**
```go
// Go запускает Rust один раз
cmd := exec.Command("toolkit.exe", "server")
stdin, _ := cmd.StdinPipe()
stdout, _ := cmd.StdoutPipe()
cmd.Start()

// Все вызовы через один процесс
for _, request := range requests {
    json.NewEncoder(stdin).Encode(request)
    scanner.Scan()
    json.Unmarshal(scanner.Bytes(), &result)
}
```

**Преимущества:**
- ✅ Нет overhead создания процесса
- ✅ ~5-10ms на вызов (вместо 50-100ms)
- ✅ Всё ещё просто

**Недостатки:**
- ⚠️ Нужно управлять процессом (restart, health check)

---

### Фаза 3: gRPC (если действительно нужно, +3-5 дней)
```
mcp-server.exe (Go) → gRPC → toolkit-server.exe (Rust)
```

**Когда нужно:**
- ❌ Если вызовы >10 раз/сек (CLI не справится)
- ❌ Если нужна distributed architecture (toolkit на другом сервере)
- ❌ Если нужна строгая типизация (proto files)

**Преимущества:**
- ✅ ~1-5ms на вызов
- ✅ Distributed architecture
- ✅ Strict typing (proto)

**Недостатки:**
- ⚠️ Сложность (proto, tonic, управление процессом)
- ⚠️ 5-7 дней разработки

---

## 💡 Моя рекомендация

### Начните с CLI (Фаза 1)

**Почему:**
1. ✅ Быстрый старт (2-3 дня)
2. ✅ Простая интеграция
3. ✅ Standalone CLI (можно использовать без MCP)
4. ✅ Легко мигрировать в gRPC позже

**План:**
```bash
# День 1-2: Создать Rust CLI toolkit
cargo new toolkit
cd toolkit
cargo add tree-sitter tree-sitter-c serde serde_json clap

# Реализовать команды:
# - toolkit analyze
# - toolkit generate-docs
# - toolkit lint

# День 3: Интеграция в Go MCP server
# - ToolkitClient (exec.Command)
# - Новые MCP инструменты

# День 4: Тестирование
# - Тест CLI standalone
# - Тест интеграции
# - Тест на ShiftCommander
```

**Если упрётесь в скорость:**
```bash
# День 5: Long-running process
# - toolkit --server mode
# - stdin/stdout communication
# - Health checks
```

**Если действительно нужен gRPC:**
```bash
# День 6-10: gRPC миграция
# - Proto files
# - tonic server
# - Go gRPC client
# - Управление процессом
```

---

## 🎯 Итоговая архитектура (Фаза 1)

```
┌─────────────────────────────────────┐
│  Qwen Code / Cursor / Cline         │
└──────────────┬──────────────────────┘
               │ STDIO (JSON-RPC)
               ▼
┌─────────────────────────────────────┐
│  mcp-server.exe (Go)                │  v0.6.0
│  - 6 существующих инструментов      │
│  + 3 новых (AST-powered):           │
│    - ast_analyze_code               │
│    - generate_doc_comments          │
│    - advanced_lint                  │
│  - ToolkitClient (exec.Command)     │
└──────────────┬──────────────────────┘
               │ exec.Command("toolkit.exe", ...)
               │ JSON stdin/stdout
               ▼
┌─────────────────────────────────────┐
│  toolkit.exe (Rust CLI)             │  v0.1.0
│  - tree-sitter AST                  │
│  - Flow analysis                    │
│  - Doc generation                   │
│  - Ходит в Qdrant/Ollama            │
│  - Команды:                         │
│    - analyze                        │
│    - generate-docs                  │
│    - lint                           │
│    - flow-analysis                  │
└──────────────┬──────────────────────┘
               │ HTTP
               ▼
┌─────────────────────────────────────┐
│  Qdrant / Ollama                    │
│  - RAG поиск                        │
│  - Эмбеддинги                       │
└─────────────────────────────────────┘
```

---

## 📝 Пример реализации

### Rust CLI (toolkit/src/main.rs):
```rust
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "toolkit")]
#[command(about = "STM32 C code analysis toolkit")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze C code for issues
    Analyze {
        #[arg(short, long)]
        file: Option<String>,
        #[arg(short, long, default_value = "json")]
        format: String,
    },
    /// Generate doc comments
    GenerateDocs {
        #[arg(short, long)]
        file: String,
    },
    /// Lint with F407 rules
    Lint {
        #[arg(short, long)]
        file: String,
    },
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Analyze { file, format } => {
            let code = read_code(file);
            let result = analyze_code(&code);
            println!("{}", serde_json::to_string(&result).unwrap());
        }
        Commands::GenerateDocs { file } => {
            let code = std::fs::read_to_string(&file).unwrap();
            let documented = generate_docs(&code);
            println!("{}", documented);
        }
        Commands::Lint { file } => {
            let code = std::fs::read_to_string(&file).unwrap();
            let violations = lint_code(&code, "f407");
            println!("{}", serde_json::to_string(&violations).unwrap());
        }
    }
}

fn analyze_code(code: &str) -> AnalysisResult {
    use tree_sitter::Parser;
    
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_c::language()).unwrap();
    let tree = parser.parse(code, None).unwrap();
    
    // AST анализ
    let violations = find_violations(&tree, code);
    
    // Опционально: RAG поиск через Qdrant
    // let context = query_qdrant("DMA CCM RAM");
    
    AnalysisResult { violations, .. }
}
```

### Go MCP Server (вызов Rust CLI):
```go
package main

import (
    "encoding/json"
    "os/exec"
    "strings"
)

type ToolkitClient struct {
    binaryPath string
}

func NewToolkitClient() *ToolkitClient {
    return &ToolkitClient{
        binaryPath: "toolkit.exe",
    }
}

func (t *ToolkitClient) Analyze(code string) (*AnalysisResult, error) {
    cmd := exec.Command(t.binaryPath, "analyze", "--format=json")
    cmd.Stdin = strings.NewReader(code)
    
    output, err := cmd.Output()
    if err != nil {
        return nil, err
    }
    
    var result AnalysisResult
    if err := json.Unmarshal(output, &result); err != nil {
        return nil, err
    }
    
    return &result, nil
}

// Использование в MCP tool
func astAnalyzeHandler(ctx context.Context, input AstAnalyzeInput) (interface{}, error) {
    toolkit := NewToolkitClient()
    result, err := toolkit.Analyze(input.Code)
    if err != nil {
        return nil, err
    }
    
    return result, nil
}
```

---

## 🚀 Финальный ответ

**Стоит ли городить gRPC на старте?**

### ❌ Нет!

**Почему:**
1. ✅ CLI достаточно быстр (50-100ms)
2. ✅ Простая интеграция (exec.Command)
3. ✅ Легко мигрировать в gRPC позже
4. ✅ Standalone CLI (можно использовать без MCP)

**Когда нужен gRPC:**
- ⚠️ Если вызовы >10 раз/сек
- ⚠️ Если нужна distributed architecture
- ⚠️ Если нужна строгая типизация

**План:**
1. **Фаза 1 (сейчас):** CLI (2-3 дня)
2. **Фаза 2 (если нужно):** Long-running process (+1 день)
3. **Фаза 3 (если действительно нужно):** gRPC (+3-5 дней)

**Итого: начните с CLI, мигрируйте по мере необходимости!** 🚀

---

**Хотите, чтобы я начал с создания Rust CLI toolkit?** 🦀

**Начнём с создания Rust toolkit?** 🦀
