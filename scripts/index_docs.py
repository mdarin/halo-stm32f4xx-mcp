#!/usr/bin/env python3
"""
Индексация документации STM32F407 в Qdrant.

Источник: kb/manifest.json (обогащённый peripheral, registers, tags)
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
import time
import requests
from pathlib import Path

# ============================================================================
# Конфигурация
# ============================================================================

KB_DIR = Path("kb")
MANIFEST_PATH = KB_DIR / "manifest.json"
INDEX_MD_PATH = KB_DIR / "INDEX.md"

QDRANT_URL = "http://localhost:6333"
COLLECTION = "stm32f407_docs"
OLLAMA_URL = "http://localhost:11434/api/embed"
EMBED_MODEL = "nomic-embed-text"

CHUNK_SIZE = 800  # символов
CHUNK_OVERLAP = 100  # символов перекрытия

# ============================================================================
# Qdrant
# ============================================================================

def create_collection():
    """Создание коллекции Qdrant с hybrid search (vector + BM25)"""
    print(f"Creating collection '{COLLECTION}'...")
    
    # Проверка существования
    resp = requests.get(f"{QDRANT_URL}/collections")
    if resp.status_code == 200:
        collections = resp.json().get("result", {}).get("collections", [])
        if any(c["name"] == COLLECTION for c in collections):
            print(f"  Collection '{COLLECTION}' already exists, deleting...")
            requests.delete(f"{QDRANT_URL}/collections/{COLLECTION}")
    
    # Создание
    resp = requests.put(f"{QDRANT_URL}/collections/{COLLECTION}", json={
        "vectors": {"size": 768, "distance": "Cosine"},
        "sparse_vectors": {"bm25": {}}
    })
    
    if resp.status_code not in [200, 201]:
        raise RuntimeError(f"Failed to create collection: {resp.text}")
    
    print(f"  ✅ Collection created")


def check_qdrant():
    """Проверка доступности Qdrant"""
    try:
        resp = requests.get(f"{QDRANT_URL}/collections", timeout=5)
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
        resp = requests.get("http://localhost:11434/api/tags", timeout=5)
        resp.raise_for_status()
        models = [m["name"] for m in resp.json().get("models", [])]
        if EMBED_MODEL not in models:
            print(f"❌ Model '{EMBED_MODEL}' not found in Ollama")
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

def index_file(file_info, stats):
    """Индексация одного файла"""
    file_name = file_info.get("file", "")
    file_path = KB_DIR / file_name
    
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
            json={"points": batch}
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
    
    # Загрузка manifest
    print(f"\n📖 Loading manifest from {MANIFEST_PATH}...")
    manifest = json.loads(MANIFEST_PATH.read_text(encoding="utf-8"))
    files = manifest.get("files", [])
    print(f"  Found {len(files)} files")
    
    # Фильтрация not_on_f407
    f407_files = [f for f in files if not f.get("not_on_f407", False)]
    skipped = len(files) - len(f407_files)
    print(f"  F407-relevant: {len(f407_files)} files")
    print(f"  Skipped (F42x/43x): {skipped} files")
    
    # Создание коллекции
    print()
    create_collection()
    
    # Индексация
    print(f"\n🔄 Indexing files...")
    stats = {"files": 0, "chunks": 0, "errors": 0, "skipped": 0}
    all_points = []
    
    for i, file_info in enumerate(f407_files):
        file_name = file_info.get("file", "")
        print(f"\r  [{i+1}/{len(f407_files)}] {file_name[:50]}...", end="", flush=True)
        
        points = index_file(file_info, stats)
        all_points.extend(points)
        stats["files"] += 1
    
    print(f"\n\n  ✅ Indexed {stats['files']} files, {stats['chunks']} chunks")
    if stats["errors"]:
        print(f"  ⚠️  Errors: {stats['errors']}")
    
    # Загрузка в Qdrant
    print(f"\n📤 Uploading {len(all_points)} points to Qdrant...")
    upload_batch(all_points)
    print(f"  ✅ Uploaded")
    
    # Индексация INDEX.md (семантический указатель)
    print(f"\n📇 Indexing INDEX.md as semantic guide...")
    if INDEX_MD_PATH.exists():
        index_content = INDEX_MD_PATH.read_text(encoding="utf-8")
        try:
            vector = embed(index_content)
            index_point = {
                "id": deterministic_id("INDEX.md"),
                "vector": vector,
                "payload": {
                    "type": "index",
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
    print(f"  Points uploaded: {len(all_points) + 1}")  # +1 for INDEX.md
    print(f"  Errors: {stats['errors']}")
    
    print(f"\n🔍 Test search:")
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
