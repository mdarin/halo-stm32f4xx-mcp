# Datasheet → Qdrant

1. 38 файлов, чанк = файл (max 26 КБ). Metadata: `doc="datasheet"`, chapter, section, title, source_file.
2. `INDEX.md` — отдельный документ `type: "index"`.
3. Три коллекции должны искаться вместе: rm0090 (регистры/протоколы) + datasheet (пины/электрика/варианты) + errata (баги + workaround). В metadata поле `doc` различает источник.
4. Фильтр `not_on_f407` относится только к rm0090_sections.
