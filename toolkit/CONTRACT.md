# Toolkit Contract — JSON stdin/stdout

Версия контракта: `1.0`. Транспорт Фазы 1: одноразовый процесс (`exec.Command`), код на stdin,
JSON-ответ на stdout, JSON-ошибки в stderr. Контакт один и тот же для будущих транспортов
(long-running stdin/stdout, gRPC) — меняется только доставка.

## 0. Общие правила

- **schema_version**: каждый запрос содержит `"schema_version": 1`; каждый успешный ответ
  содержит `"schema_version": 1`. Незнакомая старшая версия → ошибка `E_SCHEMA_UNSUPPORTED`.
- **Кодировка**: UTF-8, без BOM.
- **stdout** — только JSON результата (одна строка, без постороннего вывода).
  Диагностика/прогресс — только в stderr.
- **Коды выхода**: `0` успех; `1` ошибка (JSON-ошибка в stderr); `2` неверные аргументы CLI.
- **Позиции в коде**: везде `{ "byte_start": usize, "byte_end": usize, "line": usize (1-based),
  "col": usize (1-based, в символах) }`.

## 1. Формат ошибки (единый)

stderr, код выхода 1:

```json
{
  "schema_version": 1,
  "error": {
    "code": "E_INPUT_READ",
    "message": "failed to read file fix.c: No such file or directory",
    "position": { "byte_start": 0, "byte_end": 0, "line": 1, "col": 1 },
    "command": "analyze"
  }
}
```

`position` опционален (для синтаксических ошибок). Коды:

| Код | Значение |
|---|---|
| `E_INPUT_READ` | не удалось прочитать stdin/файл |
| `E_PARSE` | код не парсится tree-sitter (ERROR-узлы, position обязателен) |
| `E_SCHEMA_UNSUPPORTED` | неизвестная `schema_version` |
| `E_RAG_UNAVAILABLE` | Qdrant/Ollama недоступны (только rag-search; violations при этом всё равно возвращаются в stdout — деградация, см. §6) |
| `E_INTERNAL` | непредвиденная ошибка |

`E_RAG_UNAVAILABLE` — единственный код, при котором stdout содержит частичный результат
(деградация), а stderr — предупреждение; код выхода при этом `0`.

## 2. `analyze`

Запрос (stdin):

```json
{
  "schema_version": 1,
  "command": "analyze",
  "code": "void DMA1_Stream0_IRQHandler(void) { ... }"
}
```

Ответ (stdout):

```json
{
  "schema_version": 1,
  "command": "analyze",
  "source_stats": { "bytes": 4821, "lines": 173 },
  "functions": [
    {
      "name": "DMA1_Stream0_IRQHandler",
      "signature": "void DMA1_Stream0_IRQHandler(void)",
      "position": { "byte_start": 128, "byte_end": 490, "line": 7, "col": 1 },
      "attributes": [ { "name": "section", "args": [".ccmram"] } ]
    }
  ],
  "variables": [
    {
      "name": "dma_buffer",
      "type": "uint8_t",
      "storage": "static",
      "position": { "byte_start": 40, "byte_end": 96, "line": 3, "col": 1 },
      "attributes": [ { "name": "section", "args": [".ccmram"] } ]
    }
  ],
  "hal_calls": [
    {
      "callee": "HAL_DMA_Start_IT",
      "position": { "byte_start": 300, "byte_end": 344, "line": 12, "col": 5 }
    }
  ],
  "parse_errors": []
}
```

## 3. `lint` / `flow-analysis`

`lint` — правила F407 по одному файлу. `flow-analysis` — то же + трекинг использования
переменных в DMA-путях (режим `lint --flow`). Запрос (stdin):

```json
{
  "schema_version": 1,
  "command": "lint",
  "flow": false,
  "rules": ["f407"],
  "code": "static uint8_t buf[64] __attribute__((section(\".ccmram\")));"
}
```

Ответ (stdout):

```json
{
  "schema_version": 1,
  "command": "lint",
  "violations": [
    {
      "rule": "dma_buffer_in_ccmram",
      "severity": "error",
      "message": "DMA buffer 'buf' placed in .ccmram: DMA1/DMA2 cannot access CCM RAM",
      "position": { "byte_start": 0, "byte_end": 82, "line": 1, "col": 1 },
      "hint": "Move buffer to .axisram / .sram1-3, or use DMA2D/MDMA"
    }
  ],
  "flows": [
    {
      "variable": "buf",
      "uses": [
        { "kind": "declaration", "position": { "byte_start": 0, "byte_end": 82, "line": 1, "col": 1 } }
      ],
      "reaches_dma": false
    }
  ],
  "summary": { "errors": 1, "warnings": 0, "infos": 0 }
}
```

`flows` — только при `"flow": true`. Полный список правил F407 и severities — в `docs/F407_RULES.md`
(наполняется в Ф1-3 из Go-реализации `check_f407_rules`).

## 4. `generate-docs`

Запрос (stdin):

```json
{
  "schema_version": 1,
  "command": "generate-docs",
  "style": "doxygen",
  "code": "static HAL_StatusTypeDef shift_send(uint8_t *data, size_t len);\n..."
}
```

Ответ (stdout) — **вставки, а не весь файл**:

```json
{
  "schema_version": 1,
  "command": "generate-docs",
  "insertions": [
    {
      "position": { "byte_start": 0, "byte_end": 0, "line": 1, "col": 1 },
      "text": "/**\n * @brief shift_send\n * @param data ...\n * @param len ...\n * @retval HAL_StatusTypeDef\n */\n"
    }
  ],
  "functions_documented": 1,
  "functions_skipped": 0
}
```

Вставка позиционируется на строку начала сигнатуры функции без doc-комментария
(`byte_start == byte_end` — точка вставки). Функции с существующим doc-комментарием пропускаются
(`functions_skipped`).

## 5. `rag-search`

AST-анализ + обогащение контекстом из Qdrant (эмбеддинг → поиск → refs в том же JSON).
Конфигурация — env: `QDRANT_URL`, `QDRANT_API_KEY`, `OLLAMA_URL`, `COLLECTION`, `EMBED_MODEL`
(наследуются от MCP-сервера через `exec.Command`). Запрос (stdin):

```json
{
  "schema_version": 1,
  "command": "rag-search",
  "code": "void DMA1_Stream0_IRQHandler(void) { ... }",
  "rule_context": "DMA buffer placement",
  "limit": 5
}
```

Ответ (stdout):

```json
{
  "schema_version": 1,
  "command": "rag-search",
  "violations": [ { "...": "как в §3" } ],
  "rag_refs": [
    {
      "score": 0.737,
      "source": "10_10_3_DMA_functional_description__c11.md",
      "title": "DMA functional description",
      "chapter": 10,
      "section": "10.3",
      "snippet": "> [RM0090 ch.10 §10.3] ..."
    }
  ],
  "rag_degraded": false
}
```

Деградация: RAG недоступен → `violations` заполняются, `rag_refs: []`, `rag_degraded: true`,
в stderr — JSON-ошибка `E_RAG_UNAVAILABLE` (информационная), код выхода `0`.

Конфигурация (env, наследуются от MCP-сервера через `exec.Command`):

| Переменная | По умолчанию | Назначение |
|---|---|---|
| `QDRANT_URL` | `http://localhost:6333` | Адрес Qdrant (POST `/collections/{COLLECTION}/points/search`, таймаут 30 c) |
| `QDRANT_API_KEY` | — (не задан) | Заголовок `api-key`, если задан |
| `OLLAMA_URL` | `http://localhost:11434` | Адрес Ollama (POST `/api/embed`, таймаут 60 c, 3 попытки с backoff 1/2 c) |
| `COLLECTION` | `stm32f407_docs` | Коллекция Qdrant |
| `EMBED_MODEL` | `nomic-embed-text` | Модель эмбеддингов |

Запрос к эмбеддеру: `rule_context`, если задан, иначе первые 2000 символов кода.
`snippet` — первые 300 символов payload-поля `text`. Формат `/api/embed`
(`{model, input}` → `{embeddings}`), формат точки поиска
(`{vector, limit, with_payload: true, filter?}` → `{result: [{id, score, payload}]}`),
поля payload: `text`, `source_file`, `peripheral`, `chapter`, `section`, `title`.
(До Ф2-1 схемы сверялись с Go-клиентами `internal/rag/*`; с Ф2-1 эти клиенты
удалены — toolkit единственный владелец RAG-доступа.)

## 6. `rag-query` (Ф2-1)

Поиск по документации без AST-анализа — единственный RAG-канал MCP-инструмента
`query_hardware_docs`. Аргументы CLI:

```
toolkit rag-query --query "GPIO MODER register" [--peripheral TIM] [--limit 5]
```

Ответ (stdout):

```json
{
  "schema_version": 1,
  "command": "rag-query",
  "query": "GPIO MODER register",
  "peripheral": "",
  "results": [
    {
      "text": "...",
      "source": "02_GPIO_registers.md",
      "peripheral": "GPIO",
      "chapter": 8,
      "section": "8.4",
      "title": "GPIO registers",
      "score": 0.71
    }
  ]
}
```

Поля результата зеркалят payload Qdrant: `source` ← `source_file`, `chapter` —
целое. RAG недоступен → JSON-ошибка `E_RAG_UNAVAILABLE`, exit 1 (в отличие от
`rag-search`, поиск здесь — основная функция, деградации нет).

## 7. `rag-upsert` / `rag-index` (Ф2-1)

Пополнение базы знаний из Rust-стороны.

`rag-upsert` — низкоуровневая операция, точки приходят по stdin:

```json
{"points": [{"id": 42, "text": "...", "payload": {"title": "...", ...}}]}
```

Ответ: `{"schema_version": 1, "command": "rag-upsert", "upserted": 1}`.
Каждый `text` эмбеддится, в payload принудительно пишется `"text"`;
upsert батчами по 100 в `PUT /collections/{col}/points`.

`rag-index --path DIR` — индексация каталога kb: читает `manifest.json`
(схема как в `scripts/index_docs.py`: `files[].file/chapter/section/title/
peripheral/registers/tags/not_on_f407`; база знаний — `kb/stm32f4xx/
{reference,datasheet,errata}/`, в payload добавляется `doc_type`), файлы >30000 символов режет по
заголовкам `### `, id точек детерминированы (первые 8 hex SHA-256 от
`"{file}:{chunk_idx}"` — parity с python-индексатором), повторный запуск
перезаписывает точки. Ответ:

```json
{"schema_version": 1, "command": "rag-index", "files": 433, "chunks": 363, "skipped": 2, "errors": 0}
```

## 8. `--version` и semver

`toolkit --version` (аргументы CLI, не stdin):

```
toolkit 1.0.0 (contract 1.0)
```

`contract 1.0` — версия настоящего контракта. Go ToolkitClient (Ф1-6) парсит `contract N.M`
и отказывается работать при несовпадении мажорной версии.

Политика semver (обе компоненты):

- **toolkit**: версия — single source `version` в `Cargo.toml`, в `--version` попадает
  автоматически (`CARGO_PKG_VERSION`). МАЖОР = мажор контракта (изменение/удаление
  команд или формата вывода без обратной совместимости ⇒ контракт-минор-инкремент,
  менять синхронно). МИНОР — новая команда/флаг, ПАТЧ — исправления.
- **mcp-server**: константа `ServerVersion` в `mcp-server/main.go` (single source,
  попадает в serverInfo и startup-лог). МИНОР — новые инструменты, ПАТЧ — исправления.

## 9. Совместимость

- Мажорная версия контракта растёт при несовместимых изменениях схем.
- Добавление новых опциональных полей — минорная версия, обратно совместимо.
- Все поля запроса кроме `schema_version` и `command` имеют значения по умолчанию.
