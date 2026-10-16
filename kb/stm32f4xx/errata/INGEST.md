# Errata → Qdrant

1. 9 файлов, чанк = файл (max 19.7 КБ). Metadata: `doc="errata"`, chapter, section, title, source_file.
2. `not_on_f407` не нужен — все ограничения относятся к F405/407/415/417.
3. `INDEX.md` — отдельный документ `type: "index"`.
4. Retrieval-подсказка агенту: при отладке «странного» поведения периферии искать и в errata, и в rm0090 — баги кремния с workaround здесь.
