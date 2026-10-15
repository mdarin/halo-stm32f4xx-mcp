#!/usr/bin/env python3
"""
Тест поиска f407_known_issues.md в Qdrant
"""
import json
import requests

QDRANT_URL = "http://localhost:6333"
COLLECTION = "stm32f407_docs"
OLLAMA_URL = "http://localhost:11434/api/embed"
EMBED_MODEL = "nomic-embed-text"

def search(query, limit=3):
    """Поиск в Qdrant"""
    resp = requests.post(OLLAMA_URL, 
                        json={"model": EMBED_MODEL, "input": query},
                        timeout=60)
    resp.raise_for_status()
    vector = resp.json()["embeddings"][0]
    
    resp = requests.post(f"{QDRANT_URL}/collections/{COLLECTION}/points/search",
                        json={"vector": vector, "limit": limit, "with_payload": True})
    resp.raise_for_status()
    
    return resp.json()["result"]

print("=" * 60)
print("🔍 Test: DMA CCM RAM search")
print("=" * 60)

results = search("DMA CCM RAM incompatibility", limit=3)
for i, hit in enumerate(results, 1):
    payload = hit["payload"]
    print(f"\n{i}. {payload.get('title', 'N/A')}")
    print(f"   File: {payload.get('source_file', 'N/A')}")
    print(f"   Peripheral: {payload.get('peripheral', 'N/A')}")
    print(f"   Type: {payload.get('type', 'N/A')}")
    print(f"   Score: {hit['score']:.4f}")
    if "known_issues" in payload.get("source_file", ""):
        print(f"   ✅ FOUND known_issues.md!")

print("\n" + "=" * 60)
print("✅ Test completed!")
print("=" * 60)
