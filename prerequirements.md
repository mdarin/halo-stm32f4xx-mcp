# Prerequisites

Чистая машина → работающий RAG:

1. `git clone` — вся база в репо (`kb/stm32f4xx/{reference,datasheet,errata}` + `hardware_pitfalls/`)
2. `docker compose up -d qdrant`
3. `ollama pull nomic-embed-text`
4. venv + `pip install requests`
5. `python scripts/index_docs.py` — создаст коллекцию и проиндексирует всё (~413 точек, ~40–50 сек)
6. `make build` + `TOOLKIT_PATH=toolkit/target/release/toolkit` → `query_hardware_docs`

ID детерминированы (SHA-256 от имени файла) — раскладка точек воспроизводима.
