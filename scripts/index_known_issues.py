#!/usr/bin/env python3
"""
Индексация f407_known_issues.md в Qdrant
"""
import json
import requests
import hashlib
from pathlib import Path

QDRANT_URL = "http://localhost:6333"
COLLECTION = "stm32f407_docs"
OLLAMA_URL = "http://localhost:11434/api/embed"
EMBED_MODEL = "nomic-embed-text"

def main():
    print("=" * 60)
    print("📄 Indexing f407_known_issues.md")
    print("=" * 60)
    
    # Прочитать файл
    file_path = Path("kb/hardware_pitfalls/f407_known_issues.md")
    if not file_path.exists():
        print(f"❌ File not found: {file_path}")
        return False
    
    content = file_path.read_text(encoding="utf-8")
    print(f"\n📖 Read {len(content)} chars from {file_path}")
    
    # Эмбеддинг
    print("\n🧬 Requesting embedding from Ollama...")
    resp = requests.post(
        OLLAMA_URL,
        json={"model": EMBED_MODEL, "input": content},
        timeout=60
    )
    resp.raise_for_status()
    vector = resp.json()["embeddings"][0]
    print(f"  ✅ Embedding OK: {len(vector)} dims")
    
    # Детерминированный ID
    point_id = int(hashlib.sha256(str(file_path).encode()).hexdigest()[:8], 16)
    print(f"\n🆔 Point ID: {point_id}")
    
    # Загрузка в Qdrant
    print("\n📤 Uploading to Qdrant...")
    point = {
        "id": point_id,
        "vector": vector,
        "payload": {
            "source_file": str(file_path),
            "chapter": 0,
            "section": "0",
            "title": "STM32F407 Known Issues & Hardware Pitfalls",
            "peripheral": "SYSTEM",
            "registers": [],
            "tags": [
                "errata", "pitfalls", "workarounds",
                "ccm-dma", "nvic", "usb", "timer", "boot", "flash"
            ],
            "text": content,
            "chunk_id": 0,
            "not_on_f407": False,
            "schema_version": 1,
            "type": "known_issues"
        }
    }
    
    resp = requests.put(
        f"{QDRANT_URL}/collections/{COLLECTION}/points",
        json={"points": [point]}
    )
    resp.raise_for_status()
    print(f"  ✅ Upload OK")
    
    # Проверка
    print("\n🔍 Checking collection...")
    resp = requests.get(f"{QDRANT_URL}/collections/{COLLECTION}")
    total = resp.json()["result"]["points_count"]
    print(f"  Total points: {total}")
    
    print("\n" + "=" * 60)
    print("✅ f407_known_issues.md indexed successfully!")
    print("=" * 60)
    
    return True

if __name__ == "__main__":
    import sys
    success = main()
    sys.exit(0 if success else 1)
