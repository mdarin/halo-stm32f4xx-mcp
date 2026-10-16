# Toolkit CLI

`toolkit` — Rust CLI для STM32F407-разработки: AST-анализ C-кода, линтинг аппаратных ловушек, RAG-поиск по документации (Qdrant + Ollama). Самостоятельный бинарник — вызывается из любого скрипта (Python/Bash/CI) или Go MCP-сервера.

```bash
cargo build --release --manifest-path toolkit/Cargo.toml
# бинарник: toolkit/target/release/toolkit
```

## Переменные окружения

| Переменная | По умолчанию | Назначение |
|---|---|---|
| `QDRANT_URL` | `http://localhost:6333` | Адрес Qdrant |
| `QDRANT_API_KEY` | — (нет) | API-ключ Qdrant |
| `OLLAMA_URL` | `http://localhost:11434` | Адрес Ollama |
| `COLLECTION` | `stm32f407_docs` | Имя коллекции документации |
| `CONTEXT_COLLECTION` | `stm32_project_context` | Имя коллекции контекста агента |
| `EMBED_MODEL` | `nomic-embed-text` | Модель эмбеддингов |

Контракт вывода: JSON на STDOUT. RAG-команды требуют живых Qdrant и Ollama; при недоступности — exit 1, JSON `{"error": "E_RAG_UNAVAILABLE", ...}`.

---

## AST / lint (без внешних сервисов)

### `toolkit analyze`

Разбор C-исходника, AST-факты: функции, переменные, вызовы HAL, ошибки парсинга.

```bash
toolkit analyze --file src/dma.c
cat main.c | toolkit analyze
```

### `toolkit lint`

Проверка правил STM32F407 (несоответствия периферии, критичные ошибки).

```bash
toolkit lint --file src/usart.c
```

### `toolkit flow-analysis`

Аналог `lint --flow`: трекинг использования переменных в DMA-путях.

```bash
toolkit flow-analysis --file src/dma.c
```

### `toolkit generate-docs`

Генерация doc-комментариев для недокументированных функций (doxygen).

```bash
toolkit generate-docs --file src/usart.c
```

---

## RAG (Qdrant + Ollama)

### `toolkit rag-query`

Семантический поиск по базе знаний (reference RM0090 / datasheet / errata). Верхние хиты с текстом, метаданными и score.

```bash
toolkit rag-query -q "I2C peripheral silicon limitation" --limit 3
toolkit rag-query -q "DMA FIFO" --peripheral DMA
```

### `toolkit rag-search`

AST-анализ кода + обогащение документацией: для каждого найденного нарушения подтягиваются RAG-ссылки.

```bash
toolkit rag-search --file src/dma.c --rule-context "DMA config" --limit 5
cat main.c | toolkit rag-search
```

### `toolkit rag-upsert`

Вставка точек в коллекцию. JSON на STDIN: id выбирает вызывающий, `text` эмбеддится, payload мержится с `"text"`.

```bash
echo '{"points":[{"id":42,"text":"Заметка о DMA","payload":{"doc_type":"note"}}]}' | toolkit rag-upsert
```

### `toolkit rag-index`

Индексация каталога базы знаний: читает `manifest.json` (файлы >30 КБ режет по `### `), id детерминированы (первые 8 hex SHA-256 от `"{file}:{chunk_idx}"` — повторный запуск идемпотентен).

```bash
toolkit rag-index --path kb/stm32f4xx/reference
QDRANT_URL=http://127.0.0.1:6333 COLLECTION=stm32f407_docs toolkit rag-index --path kb/stm32f4xx/errata
```

Подробный контракт форматов — `toolkit/CONTRACT.md`.

---

## Контекст агента (Qdrant + Ollama)

Отдельная коллекция `stm32_project_context` (CONTRACT.md §10): сводка проекта, история модификаций, предпочтения пользователя. id точки = первые 8 hex SHA-256 от `"{project_path}:{ctx_key}"` — повторный ctx-save того же ключа перезаписывает точку.

### `toolkit ctx-save`

Сохранение записей контекста. JSON на STDIN: `project_path` + массив `entries` (`key`, `text`, опционально `doc_type`, `peripherals`, `interrupts`, `user_preferences`, `modification`). Коллекция создаётся при отсутствии.

```bash
echo '{"project_path":"/abs/path/project","entries":[{"key":"summary","text":"TIM3 = PWM, USART2 = telemetry","peripherals":["TIM3","USART2"]}]}' | toolkit ctx-save
```

### `toolkit ctx-load`

Чтение сохранённого контекста проекта (или одной записи по `--key`).

```bash
toolkit ctx-load --project /abs/path/project
toolkit ctx-load --project /abs/path/project --key prefs
```

### `toolkit ctx-search`

Семантический поиск по контексту проекта (фильтр по `project_path`).

```bash
toolkit ctx-search --project /abs/path/project -q "как настроен PWM" --limit 3
```

### `toolkit ctx-extract`

Автосбор контекста проекта: периферия из `*.ioc` (`Mcu.IP*`), MCU (`Mcu.Name`), IRQ-обработчики (AST-скан `_IRQHandler`), USER CODE блоки и файлы с ними. Если Qdrant доступен — заполняет `new_peripherals`/`new_interrupts` (отличия от сохранённого контекста); если нет — поля опущены, команда не падает. Вывод пригоден для пайпа в `ctx-save` (поля `peripherals`/`interrupts` совпадают по именам).

```bash
toolkit ctx-extract --project /abs/path/project
```
