#!/usr/bin/env python3
"""
Индексация документации STM32F407 в Qdrant.

Источники: kb/stm32f4xx/{reference,datasheet,errata}/manifest.json (обогащённый peripheral, registers, tags)
Цель: Qdrant коллекция stm32f407_docs для RAG-поиска

Использование:
    python scripts/index_docs.py

Требования:
    - Qdrant запущен (docker compose up -d qdrant)
    - Ollama запущена (ollama serve)
    - Модель nomic-embed-text загружена (ollama pull nomic-embed-text)
"""

import json
import hashlib
import os
import time
import sys
import requests
from pathlib import Path

# Установка кодировки UTF-8 для вывода (Windows)
if sys.platform == 'win32':
    sys.stdout.reconfigure(encoding='utf-8')
    sys.stderr.reconfigure(encoding='utf-8')

# ============================================================================
# Конфигурация
# ============================================================================

KB_ROOT = Path("kb/stm32f4xx")
# doc_type -> подкаталог kb/stm32f4xx/<name>/ с manifest.json
DOC_DIRS = {
    "reference": KB_ROOT / "reference",
    "datasheet": KB_ROOT / "datasheet",
    "errata": KB_ROOT / "errata",
}
QDRANT_URL = os.environ.get("QDRANT_URL", "http://localhost:6333")
QDRANT_API_KEY = os.environ.get("QDRANT_API_KEY", "")
COLLECTION = os.environ.get("COLLECTION", "stm32f407_docs")
OLLAMA_URL = os.environ.get("OLLAMA_URL", "http://localhost:11434") + "/api/embed"
EMBED_MODEL = os.environ.get("EMBED_MODEL", "nomic-embed-text")

CHUNK_SIZE = 800  # символов
CHUNK_OVERLAP = 100  # символов перекрытия

QDRANT_HEADERS = {"api-key": QDRANT_API_KEY} if QDRANT_API_KEY else {}

# ============================================================================
# Qdrant
# ============================================================================

def create_collection():
    """Создание коллекции Qdrant с hybrid search (vector + BM25)"""
    print(f"Checking collection '{COLLECTION}'...")
    
    # Проверка существования
    resp = requests.get(f"{QDRANT_URL}/collections", headers=QDRANT_HEADERS)
    if resp.status_code == 200:
        collections = resp.json().get("result", {}).get("collections", [])
        if any(c["name"] == COLLECTION for c in collections):
            print(f"  ✅ Collection '{COLLECTION}' already exists")
            return
    
    # Создание
    print(f"  Creating collection '{COLLECTION}'...")
    resp = requests.put(f"{QDRANT_URL}/collections/{COLLECTION}", json={
        "vectors": {"size": 768, "distance": "Cosine"},
        "sparse_vectors": {"bm25": {}}
    }, headers=QDRANT_HEADERS)
    
    if resp.status_code not in [200, 201]:
        raise RuntimeError(f"Failed to create collection: {resp.text}")
    
    print(f"  ✅ Collection created")


def check_qdrant():
    """Проверка доступности Qdrant"""
    try:
        resp = requests.get(f"{QDRANT_URL}/collections", headers=QDRANT_HEADERS, timeout=5)
        resp.raise_for_status()
        return True
    except Exception as e:
        print(f"❌ Qdrant not available at {QDRANT_URL}: {e}")
        print("   Run: docker compose up -d qdrant")
        return False

# ============================================================================
# Ollama
# ============================================================================

def embed(text, retries=3):
    """Получение эмбеддинга через Ollama с retry"""
    for attempt in range(retries):
        try:
            resp = requests.post(
                OLLAMA_URL,
                json={"model": EMBED_MODEL, "input": text},
                timeout=60
            )
            resp.raise_for_status()
            return resp.json()["embeddings"][0]
        except Exception as e:
            if attempt == retries - 1:
                raise RuntimeError(f"Ollama unavailable after {retries} attempts: {e}")
            time.sleep(1 * (attempt + 1))


def check_ollama():
    """Проверка доступности Ollama"""
    try:
        resp = requests.get(OLLAMA_URL.replace("/api/embed", "/api/tags"), timeout=5)
        resp.raise_for_status()
        models = [m["name"] for m in resp.json().get("models", [])]
        # Проверка: модель может быть "nomic-embed-text" или "nomic-embed-text:latest"
        if not any(EMBED_MODEL in m for m in models):
            print(f"❌ Model '{EMBED_MODEL}' not found in Ollama")
            print(f"   Available models: {models}")
            print(f"   Run: ollama pull {EMBED_MODEL}")
            return False
        return True
    except Exception as e:
        print(f"❌ Ollama not available: {e}")
        print("   Run: ollama serve")
        return False

# ============================================================================
# Чанкинг
# ============================================================================

def deterministic_id(path, chunk_idx=0):
    """Детерминированный ID на основе SHA-256"""
    hash_input = f"{path}:{chunk_idx}"
    return int(hashlib.sha256(hash_input.encode()).hexdigest()[:8], 16)


def chunk_text(text, chunk_size=CHUNK_SIZE, overlap=CHUNK_OVERLAP):
    """Разбиение текста на чанки с перекрытием"""
    if len(text) <= chunk_size:
        return [text]
    
    chunks = []
    start = 0
    while start < len(text):
        end = start + chunk_size
        chunk = text[start:end]
        chunks.append(chunk)
        start = end - overlap
    
    return chunks


def chunk_by_headers(content, file_info):
    """Разбиение по заголовкам ### с сохранением контекста"""
    lines = content.split('\n')
    chunks = []
    current_chunk = []
    current_header = ""
    
    for line in lines:
        if line.startswith('### '):
            # Сохраняем предыдущий чанк
            if current_chunk:
                chunk_text = '\n'.join(current_chunk)
                if len(chunk_text.strip()) > 50:
                    chunks.append((current_header, chunk_text))
            
            current_header = line
            current_chunk = [line]
        else:
            current_chunk.append(line)
    
    # Последний чанк
    if current_chunk:
        chunk_text = '\n'.join(current_chunk)
        if len(chunk_text.strip()) > 50:
            chunks.append((current_header, chunk_text))
    
    return chunks

# ============================================================================
# Индексация
# ============================================================================

def index_file(file_info, stats, doc_dir, doc_type):
    """Индексация одного файла"""
    file_name = file_info.get("file", "")
    file_path = doc_dir / file_name
    
    if not file_path.exists():
        stats["errors"] += 1
        return []
    
    content = file_path.read_text(encoding="utf-8")
    
    # Пропуск пустых файлов
    if len(content.strip()) < 50:
        stats["skipped"] += 1
        return []
    
    # Метаданные из manifest
    metadata = {
        "source_file": file_name,
        "doc_type": doc_type,
        "chapter": file_info.get("chapter"),
        "section": file_info.get("section"),
        "title": file_info.get("title"),
        "peripheral": file_info.get("peripheral"),
        "registers": file_info.get("registers", []),
        "tags": file_info.get("tags", []),
        "not_on_f407": file_info.get("not_on_f407", False),
        "schema_version": 1
    }
    
    # Разбиение на чанки
    if len(content) > 30000:
        # Большие файлы — по заголовкам ###
        header_chunks = chunk_by_headers(content, file_info)
        points = []
        for i, (header, chunk) in enumerate(header_chunks):
            if len(chunk.strip()) < 50:
                continue
            
            # Добавляем контекстную шапку
            full_text = f"{file_info.get('title', '')}\n{header}\n{chunk}"
            
            try:
                vector = embed(full_text)
                points.append({
                    "id": deterministic_id(file_name, i),
                    "vector": vector,
                    "payload": {
                        **metadata,
                        "text": chunk,
                        "chunk_id": i,
                        "chunk_header": header
                    }
                })
                stats["chunks"] += 1
            except Exception as e:
                print(f"\n  ⚠️  Embed error for {file_name} chunk {i}: {e}")
                stats["errors"] += 1
        
        return points
    else:
        # Маленькие файлы — один чанк
        try:
            vector = embed(content)
            points = [{
                "id": deterministic_id(file_name),
                "vector": vector,
                "payload": {
                    **metadata,
                    "text": content,
                    "chunk_id": 0
                }
            }]
            stats["chunks"] += 1
            return points
        except Exception as e:
            print(f"\n  ⚠️  Embed error for {file_name}: {e}")
            stats["errors"] += 1
            return []


def upload_batch(points, batch_size=100):
    """Загрузка точек в Qdrant батчами"""
    total = len(points)
    for i in range(0, total, batch_size):
        batch = points[i:i+batch_size]
        resp = requests.put(
            f"{QDRANT_URL}/collections/{COLLECTION}/points",
            json={"points": batch},
            headers=QDRANT_HEADERS
        )
        if resp.status_code not in [200, 201]:
            raise RuntimeError(f"Upload failed: {resp.text}")


def index_all():
    """Основная функция индексации"""
    print("=" * 60)
    print("📚 STM32F407 Documentation Indexer")
    print("=" * 60)
    
    # Проверки
    print("\n🔍 Checking dependencies...")
    if not check_qdrant():
        return
    print("  ✅ Qdrant OK")
    
    if not check_ollama():
        return
    print("  ✅ Ollama OK")
    
    # Загрузка manifest по типам документации
    doc_sets = []
    for doc_type, doc_dir in DOC_DIRS.items():
        manifest_path = doc_dir / "manifest.json"
        if not manifest_path.exists():
            print(f"  ⚠️  {manifest_path} не найден, пропуск '{doc_type}'")
            continue
        manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
        files = manifest.get("files", [])
        print(f"  📖 {doc_type}: {len(files)} файлов ({manifest_path})")
        doc_sets.append((doc_type, doc_dir, files))
    
    # Фильтрация not_on_f407 (только reference размечен)
    all_files = []
    for doc_type, doc_dir, files in doc_sets:
        f407_files = [f for f in files if not f.get("not_on_f407", False)]
        skipped = len(files) - len(f407_files)
        print(f"  {doc_type}: F407-relevant {len(f407_files)}, skipped {skipped}")
        for f in f407_files:
            all_files.append((doc_type, doc_dir, f))

    # Создание коллекции
    print()
    create_collection()

    # Проверка уже проиндексированных файлов
    print(f"\n🔍 Checking existing points...")
    indexed_files = set()
    missing_doc_type = set()
    try:
        resp = requests.post(
            f"{QDRANT_URL}/collections/{COLLECTION}/points/scroll",
            json={"limit": 1000, "with_payload": {"include": ["source_file", "doc_type"]}},
            headers=QDRANT_HEADERS
        )
        if resp.status_code == 200:
            points = resp.json().get("result", {}).get("points", [])
            for point in points:
                source = point.get("payload", {}).get("source_file")
                if source:
                    indexed_files.add(source)
                    if not point.get("payload", {}).get("doc_type"):
                        missing_doc_type.add(source)
            print(f"  Found {len(indexed_files)} already indexed files")
            if missing_doc_type:
                print(f"  Found {len(missing_doc_type)} files without doc_type (backfill planned)")
    except Exception as e:
        print(f"  ⚠️  Could not check existing points: {e}")

    # Фильтрация уже проиндексированных
    files_to_index = [
        (doc_type, doc_dir, f) for doc_type, doc_dir, f in all_files
        if f.get("file") not in indexed_files
    ]
    print(f"  Files to index: {len(files_to_index)}")

    if not files_to_index:
        print("\n✅ All files already indexed!")
        if missing_doc_type:
            backfill_doc_type(missing_doc_type, doc_sets)
        return

    # Индексация
    print(f"\n🔄 Indexing files...")
    stats = {"files": 0, "chunks": 0, "errors": 0, "skipped": 0}
    by_type = {t: {"files": 0, "chunks": 0} for t, _, _ in doc_sets}
    all_points = []

    for i, (doc_type, doc_dir, file_info) in enumerate(files_to_index):
        file_name = file_info.get("file", "")
        print(f"\r  [{i+1}/{len(files_to_index)}] {file_name[:50]}...", end="", flush=True)

        points = index_file(file_info, stats, doc_dir, doc_type)
        all_points.extend(points)
        stats["files"] += 1
        by_type[doc_type]["files"] += 1
        by_type[doc_type]["chunks"] += len(points)

        # Промежуточная загрузка каждые 50 файлов
        if len(all_points) >= 50:
            upload_batch(all_points)
            all_points = []

    print(f"\n\n  ✅ Indexed {stats['files']} files, {stats['chunks']} chunks")
    for doc_type, s in by_type.items():
        print(f"    {doc_type}: {s['files']} files, {s['chunks']} chunks")
    if stats["errors"]:
        print(f"  ⚠️  Errors: {stats['errors']}")
    
    # Загрузка оставшихся
    if all_points:
        print(f"\n📤 Uploading remaining {len(all_points)} points to Qdrant...")
        upload_batch(all_points)
        print(f"  ✅ Uploaded")

    # Backfill doc_type для старых точек без него
    if missing_doc_type:
        backfill_doc_type(missing_doc_type, doc_sets)

    # Индексация INDEX.md (семантический указатель, reference)
    print(f"\n📇 Indexing INDEX.md as semantic guide...")
    index_md_path = DOC_DIRS["reference"] / "INDEX.md"
    if index_md_path.exists():
        index_content = index_md_path.read_text(encoding="utf-8")
        try:
            vector = embed(index_content)
            index_point = {
                "id": deterministic_id("INDEX.md"),
                "vector": vector,
                "payload": {
                    "type": "index",
                    "doc_type": "index",
                    "text": index_content,
                    "source_file": "INDEX.md",
                    "title": "RM0090 Semantic Index — Task → Chapter Guide",
                    "tags": ["index", "guide", "navigation"],
                    "schema_version": 1
                }
            }
            upload_batch([index_point])
            print(f"  ✅ INDEX.md indexed")
        except Exception as e:
            print(f"  ⚠️  Failed to index INDEX.md: {e}")
    
    # Итог
    print("\n" + "=" * 60)
    print("✅ DONE!")
    print("=" * 60)
    print(f"\n📊 Statistics:")
    print(f"  Files indexed: {stats['files']}")
    print(f"  Chunks created: {stats['chunks']}")
    print(f"  Errors: {stats['errors']}")

    print(f"\n🔍 Test search:")


def backfill_doc_type(missing_doc_type, doc_sets):
    """Добавить doc_type в payload уже проиндексированных точек (без переэмбеддинга)."""
    print(f"\n🏷️  Backfilling doc_type for {len(missing_doc_type)} files...")
    name_to_type = {}
    for doc_type, _, files in doc_sets:
        for f in files:
            name_to_type[f.get("file", "")] = doc_type
    updated = 0
    for source in missing_doc_type:
        doc_type = name_to_type.get(source)
        if not doc_type:
            continue
        resp = requests.post(
            f"{QDRANT_URL}/collections/{COLLECTION}/points/payload",
            json={
                "payload": {"doc_type": doc_type},
                "filter": {"must": [{"key": "source_file", "match": {"value": source}}]}
            },
            headers=QDRANT_HEADERS
        )
        if resp.status_code in [200, 201]:
            updated += 1
        else:
            print(f"  ⚠️  Failed to set doc_type for {source}: {resp.text}")
    print(f"  ✅ doc_type set for {updated} files")
    print(f"  curl {QDRANT_URL}/collections/{COLLECTION}/points/search \\")
    print(f"    -H 'Content-Type: application/json' \\")
    print(f"    -d '{{\"vector\": [0.1, 0.2, ...], \"limit\": 5}}'")
    
    print(f"\n📖 Next steps:")
    print(f"  1. Test search via Qdrant API")
    print(f"  2. Implement MCP tool 'query_hardware_docs'")
    print(f"  3. Create f407_known_issues.md")


if __name__ == "__main__":
    try:
        index_all()
    except KeyboardInterrupt:
        print("\n\n⚠️  Interrupted by user")
    except Exception as e:
        print(f"\n\n❌ Fatal error: {e}")
        import traceback
        traceback.print_exc()
