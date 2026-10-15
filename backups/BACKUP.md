# 📦 Qdrant Backup System — Инструкция

## 🎯 Назначение

Эта система защищает данные Qdrant (коллекция `stm32f407_docs`, 363 точки) от потери при:
- Случайном удалении `./data/qdrant`
- Выполнении `docker compose down -v` (удаление volume)
- Сбоях Docker или файловой системы
- Необходимости переноса данных на другую машину

---

## 🚀 Быстрый старт

### Создать backup:
```bash
venv\bin\python.exe scripts/qdrant_backup.py backup
```

### Восстановить из последнего backup:
```bash
venv\bin\python.exe scripts/qdrant_backup.py restore
```

### Список всех backup'ов:
```bash
venv\bin\python.exe scripts/qdrant_backup.py list
```

---

## 📋 Команды

### `backup` — экспорт коллекции в JSON

**Что делает:**
- Экспортирует все точки из коллекции `stm32f407_docs`
- Сохраняет векторы (768 dims) + метаданные (peripheral, registers, tags, text)
- Создаёт файл `backups/qdrant/qdrant_backup_YYYYMMDD_HHMMSS.json`

**Пример:**
```bash
venv\bin\python.exe scripts/qdrant_backup.py backup
```

**Вывод:**
```
📦 Qdrant Backup
============================================================
🔍 Checking collection 'stm32f407_docs'...
  ✅ Found 363 points
📥 Exporting points...
  Exported 363/363 points...
  ✅ Exported 363 points
💾 Saving to backups\qdrant\qdrant_backup_20261014_205531.json...
  ✅ Backup created: qdrant_backup_20261014_205531.json (9.05 MB)
```

---

### `restore` — импорт коллекции из JSON

**Что делает:**
- Загружает backup из JSON файла
- Удаляет существующую коллекцию (если есть)
- Создаёт новую коллекцию с теми же параметрами
- Импортирует все точки (векторы + метаданные)

**Использование:**
```bash
# Из последнего backup
venv\bin\python.exe scripts/qdrant_backup.py restore

# Из конкретного файла
venv\bin\python.exe scripts/qdrant_backup.py restore backups/qdrant/qdrant_backup_20261014_205531.json
```

**Вывод:**
```
📦 Qdrant Restore
============================================================
🔍 Finding latest backup in backups/qdrant...
  ✅ Found: qdrant_backup_20261014_205531.json
📂 Loading backup...
  ✅ Loaded 363 points (vector size: 768)
🔍 Checking collection 'stm32f407_docs'...
  ⚠️  Collection 'stm32f407_docs' already exists
  Deleting...
  Creating collection 'stm32f407_docs'...
  ✅ Collection created
📤 Importing points...
  Imported 363/363 points...
  ✅ Imported 363 points
```

---

### `list` — список всех backup'ов

**Что делает:**
- Сканирует папку `backups/qdrant/`
- Показывает список backup'ов с метаданными (дата, количество точек, размер)

**Пример:**
```bash
venv\bin\python.exe scripts/qdrant_backup.py list
```

**Вывод:**
```
📦 Qdrant Backups
============================================================
📁 Found 1 backup(s) in backups\qdrant:

  📦 qdrant_backup_20261014_205531.json
     Date: 2026-10-14 20:55:31
     Points: 363
     Size: 9.05 MB
```

---

## 📁 Структура файлов

```
halo-f407-mcp/
├── backups/
│   ├── BACKUP.md              ← этот файл (инструкция)
│   └── qdrant/
│       ├── qdrant_backup_20261014_205531.json  (9.05 MB)
│       ├── qdrant_backup_20261015_120000.json  (будущие backup'ы)
│       └── ...
├── data/
│   └── qdrant/                ← Docker volume (bind mount)
│       ├── collections/       ← реальные данные Qdrant
│       ├── aliases/
│       └── raft_state.json
├── scripts/
│   └── qdrant_backup.py       ← скрипт backup/restore
└── .gitignore                 ← backups/ разрешён для git
```

---

## ⚠️ Предостережения

### ✅ Безопасные операции (данные сохраняются)

| Операция | Данные | Комментарий |
|----------|--------|-------------|
| `docker stop stm32_qdrant` | ✅ Сохраняются | Остановка контейнера |
| `docker restart stm32_qdrant` | ✅ Сохраняются | Перезапуск контейнера |
| `docker compose down` | ✅ Сохраняются | Удаление контейнера (volume остаётся) |
| `docker compose up -d` | ✅ Загружаются | Пересоздание контейнера |
| `rm -rf backups/` | ❌ **Удаляются!** | Удаление backup'ы |

### ❌ Опасные операции (данные будут потеряны)

| Операция | Данные | Комментарий |
|----------|--------|-------------|
| `docker compose down -v` | ❌ **Удаляются!** | Удаление контейнера + volume |
| `rm -rf ./data/qdrant` | ❌ **Удаляются!** | Ручное удаление volume |
| `docker volume rm stm32_qdrant` | ❌ **Удаляются!** | Удаление named volume |

**Правило:** Перед выполнением опасных операций — **всегда создавайте backup!**

```bash
# Сначала backup
venv\bin\python.exe scripts/qdrant_backup.py backup

# Потом опасная операция
docker compose down -v

# Восстановление
docker compose up -d
venv\bin\python.exe scripts/qdrant_backup.py restore
```

---

## 🔧 Технические детали

### Формат backup файла

```json
{
  "collection": "stm32f407_docs",
  "timestamp": "20261014_205531",
  "points_count": 363,
  "vector_size": 768,
  "points": [
    {
      "id": 123456789,
      "vector": [0.123, -0.456, ...],  // 768 dims
      "payload": {
        "source_file": "08_8_3_GPIO_functional_description.md",
        "chapter": 8,
        "section": "8.3",
        "title": "GPIO functional description",
        "peripheral": "GPIO",
        "registers": ["GPIOX_MODER", "GPIOX_OTYPER", ...],
        "tags": ["gpio", "digital-io", "pin-config"],
        "text": "## 8.3 GPIO functional description\n...",
        "chunk_id": 0,
        "not_on_f407": false,
        "schema_version": 1
      }
    },
    ...
  ]
}
```

### Размер backup'а

- **363 точки** × (768 dims × 4 bytes + ~500 bytes payload) ≈ **9 MB**
- Рекомендуется хранить backup'и в git (разрешено `.gitignore`)
- При необходимости — сжимать: `gzip backups/qdrant/*.json`

---

## 🔄 Автоматизация (опционально)

### Windows Task Scheduler (еженедельный backup)

1. Открыть Task Scheduler
2. Create Basic Task
3. Trigger: Weekly (например, воскресенье, 02:00)
4. Action: Start a program
   - Program: `C:\Users\mdari\projects\mcp\halo-f407-mcp\venv\bin\python.exe`
   - Arguments: `scripts/qdrant_backup.py backup`
   - Start in: `C:\Users\mdari\projects\mcp\halo-f407-mcp`

### Linux/Mac cron (еженедельный backup)

```bash
# crontab -e
0 2 * * 0 cd /path/to/halo-f407-mcp && venv/bin/python scripts/qdrant_backup.py backup
```

---

## 📊 Статистика

**Первый backup (2026-10-14):**
- 📦 Файл: `qdrant_backup_20261014_205531.json`
- 📊 Точки: 363
- 💾 Размер: 9.05 MB
- 📅 Дата: 2026-10-14 20:55:31

**Коллекция `stm32f407_docs`:**
- 📊 Точки: 363 (все F407-relevant файлы)
- 🧬 Вектор: 768 dims (nomic-embed-text)
- 🏷️ Метаданные: peripheral, registers, tags, chapter, section
- 📁 Источник: `kb/` (431 MD файл, 73 исключены как not_on_f407)

---

## 🆘 Восстановление после сбоя

### Сценарий 1: Удалён `./data/qdrant`

```bash
# 1. Пересоздать контейнер
docker compose up -d qdrant

# 2. Восстановить из backup
venv\bin\python.exe scripts/qdrant_backup.py restore

# 3. Проверить
curl http://localhost:6333/collections/stm32f407_docs
```

### Сценарий 2: Удалён backup + data

```bash
# Переиндексировать заново
venv\bin\python.exe scripts/index_docs.py

# Это займёт ~15 минут (363 файла × эмбеддинг)
```

### Сценарий 3: Перенос на другую машину

```bash
# 1. На старой машине: создать backup
venv\bin\python.exe scripts/qdrant_backup.py backup

# 2. Скопировать backups/qdrant/ на новую машину (git, rsync, etc.)

# 3. На новой машине: поднять Qdrant
docker compose up -d qdrant

# 4. Восстановить из backup
venv\bin\python.exe scripts/qdrant_backup.py restore
```

---

## 📞 Поддержка

**Проблемы:**
- ❓ `ModuleNotFoundError: requests` → использовать `venv\bin\python.exe`
- ❓ `Connection refused` → проверить, что Qdrant запущен (`docker ps`)
- ❓ `Collection not found` → запустить `index_docs.py` или `restore`

**Полезные команды:**
```bash
# Проверить Qdrant
curl http://localhost:6333/collections

# Проверить коллекцию
curl http://localhost:6333/collections/stm32f407_docs

# Список backup'ов
venv\bin\python.exe scripts/qdrant_backup.py list
```

---

**Последнее обновление:** 2026-10-14  
**Скрипт:** `scripts/qdrant_backup.py`  
**Backup'и:** `backups/qdrant/`  
**Статус:** ✅ Работает, 1 backup создан (363 точки, 9.05 MB)
