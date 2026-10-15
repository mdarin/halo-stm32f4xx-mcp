# Интеграция MCP сервера с AI-агентами

## 🔧 Qwen Code

### Автоматическая настройка (рекомендуется)

Qwen Code автоматически обнаруживает MCP серверы из `.qwen/settings.json`.

**Создайте файл `.qwen/settings.json` в корне проекта:**

```json
{
  "mcpServers": {
    "stm32f407": {
      "command": "C:\\Users\\mdari\\projects\\mcp\\halo-f407-mcp\\mcp-server\\mcp-server.exe",
      "args": [],
      "env": {
        "QDRANT_URL": "http://localhost:6333",
        "OLLAMA_URL": "http://localhost:11434",
        "COLLECTION": "stm32f407_docs",
        "EMBED_MODEL": "nomic-embed-text"
      }
    }
  }
}
```

**Проверка работы:**
```bash
# В Qwen Code чате введите:
/mcp

# Должен показать:
# stm32f407: connected (4 tools)
```

---

## 🔧 Cursor

### Настройка через `.cursor/mcp.json`

**Создайте файл `.cursor/mcp.json` в корне проекта:**

```json
{
  "mcpServers": {
    "stm32f407": {
      "command": "C:\\Users\\mdari\\projects\\mcp\\halo-f407-mcp\\mcp-server\\mcp-server.exe",
      "args": [],
      "env": {
        "QDRANT_URL": "http://localhost:6333",
        "OLLAMA_URL": "http://localhost:11434"
      }
    }
  }
}
```

**Проверка работы:**
1. Откройте Cursor
2. Перейдите в Settings → MCP
3. Должен появиться `stm32f407` с 4 инструментами

---

## 🔧 Cline (VS Code)

### Настройка через `.cline/mcp.json`

**Создайте файл `.cline/mcp.json` в корне проекта:**

```json
{
  "mcpServers": {
    "stm32f407": {
      "type": "stdio",
      "command": "C:\\Users\\mdari\\projects\\mcp\\halo-f407-mcp\\mcp-server\\mcp-server.exe",
      "args": [],
      "env": {
        "QDRANT_URL": "http://localhost:6333",
        "OLLAMA_URL": "http://localhost:11434"
      }
    }
  }
}
```

---

## 📋 Доступные инструменты

После подключения агенту будут доступны 4 инструмента:

### 1. `query_hardware_docs`
**Назначение:** Поиск по документации RM0090

**Пример использования:**
```
Найди информацию о регистре GPIO MODER
```

**Параметры:**
- `query` (string) — поисковый запрос
- `peripheral` (string, optional) — фильтр по периферии (GPIO, TIM, DMA, etc.)
- `limit` (int, optional) — количество результатов (по умолчанию: 5)

---

### 2. `check_f407_rules`
**Назначение:** Проверка C кода на критические ошибки STM32F407

**Пример использования:**
```
Проверь этот код на ошибки F407:
uint8_t dma_buffer[1024] __attribute__((section(".ccmram")));
```

**Правила:**
- CCM RAM + DMA несовместимость (critical)
- NVIC priority grouping (warning)
- USB/SDIO требуют 48 MHz (critical)
- Flash ROP Level 2 (critical, необратимо!)

---

### 3. `cubemx_read_user_blocks`
**Назначение:** Чтение USER CODE блоков из CubeMX проекта

**Пример использования:**
```
Прочитай все USER CODE блоки из проекта C:/Users/mdari/projects/ShiftCommander
```

**Параметры:**
- `project_root` (string) — корень проекта
- `file_path` (string, optional) — конкретный файл

---

### 4. `cubemx_insert_code`
**Назначение:** Безопасная вставка кода в USER CODE блоки

**Пример использования:**
```
Вставь код в блок 'Includes' файла main.c:
#include "my_header.h"
```

**Параметры:**
- `file_path` (string) — путь к файлу
- `block_name` (string) — имя блока (Includes, PV, 2, 4, etc.)
- `code` (string) — код для вставки
- `mode` (string, optional) — режим: append (по умолчанию) или replace

**Безопасность:**
- ✅ НИКОГДА не пишет вне USER CODE BEGIN/END маркеров
- ✅ Проверяет существование блока перед вставкой
- ✅ Атомарная запись (нет частичных записей при ошибках)

---

## 🚀 Проверка работы

### 1. Запустите Qdrant
```bash
docker compose up -d qdrant
```

### 2. Проверьте доступность
```bash
curl http://localhost:6333/collections
# Должен показать: stm32f407_docs (364 точки)
```

### 3. Запустите MCP сервер вручную (для теста)
```bash
cd mcp-server
.\mcp-server.exe
```

### 4. В AI-агенте (Qwen Code, Cursor, Cline)
```
# Проверьте подключение
/mcp

# Или попробуйте использовать инструмент
Найди информацию о GPIO MODER в документации STM32F407
```

---

## ⚠️ Известные проблемы

### Docker Desktop не запущен
**Симптом:** Qdrant недоступен
**Решение:** Запустите Docker Desktop вручную

### Порт 6333 занят
**Симптом:** `docker compose up` не работает
**Решение:**
```cmd
netstat -ano | findstr :6333
taskkill /PID <pid> /F
```

### MSYS2 Go конфликт
**Симптом:** Ошибка компиляции `version mismatch`
**Решение:**
```cmd
set PATH=C:\Program Files\Go\bin;%PATH%
go build -o mcp-server.exe .
```

---

## 📞 Поддержка

**Документация:**
- `mcp-server/README.md` — полная документация MCP сервера
- `backups/BACKUP.md` — backup система Qdrant
- `TASK.md` — полный план проекта

**Логи:**
- MCP сервер пишет логи в STDERR
- Qdrant: `docker logs stm32_qdrant`

**Тесты:**
```bash
# Тест STDIO транспорта
venv\bin\python.exe scripts/test_mcp_server.py

# Тест check_f407_rules
venv\bin\python.exe scripts/test_check_f407_rules.py

# Тест cubemx_read_user_blocks
venv\bin\python.exe scripts/test_cubemx_read_user_blocks.py
```

---

**Последнее обновление:** 2026-10-15  
**Версия MCP сервера:** 0.4.0  
**Статус:** 4 инструмента готовы (query_hardware_docs, check_f407_rules, cubemx_read_user_blocks, cubemx_insert_code)
