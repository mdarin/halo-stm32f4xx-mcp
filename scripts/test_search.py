#!/usr/bin/env python3
"""
Тестовый поиск в Qdrant для проверки качества RAG
"""
import json
import requests
from pathlib import Path

QDRANT_URL = "http://localhost:6333"
COLLECTION = "stm32f407_docs"
OLLAMA_URL = "http://localhost:11434/api/embed"
EMBED_MODEL = "nomic-embed-text"

def search(query, limit=5, filters=None):
    """Поиск в Qdrant"""
    # Получение эмбеддинга
    resp = requests.post(OLLAMA_URL, 
                        json={"model": EMBED_MODEL, "input": query},
                        timeout=60)
    resp.raise_for_status()
    vector = resp.json()["embeddings"][0]
    
    # Поиск в Qdrant
    payload = {
        "vector": vector,
        "limit": limit,
        "with_payload": True
    }
    
    if filters:
        payload["filter"] = filters
    
    resp = requests.post(f"{QDRANT_URL}/collections/{COLLECTION}/points/search",
                        json=payload)
    resp.raise_for_status()
    
    return resp.json()["result"]

def main():
    print("=" * 60)
    print("🔍 RAG Search Test")
    print("=" * 60)
    
    # Тест 1: GPIO MODER register
    print("\n📝 Test 1: 'GPIO MODER register'")
    results = search("GPIO MODER register configuration", limit=3)
    for i, hit in enumerate(results, 1):
        payload = hit["payload"]
        print(f"\n  {i}. {payload.get('title', 'N/A')}")
        print(f"     File: {payload.get('source_file', 'N/A')}")
        print(f"     Peripheral: {payload.get('peripheral', 'N/A')}")
        print(f"     Chapter: {payload.get('chapter', 'N/A')}")
        print(f"     Score: {hit['score']:.4f}")
        print(f"     Text preview: {payload.get('text', '')[:150]}...")
    
    # Тест 2: TIM1 BDTR dead-time
    print("\n\n📝 Test 2: 'TIM1 BDTR dead-time register'")
    results = search("TIM1 BDTR dead-time configuration", limit=3)
    for i, hit in enumerate(results, 1):
        payload = hit["payload"]
        print(f"\n  {i}. {payload.get('title', 'N/A')}")
        print(f"     File: {payload.get('source_file', 'N/A')}")
        print(f"     Peripheral: {payload.get('peripheral', 'N/A')}")
        print(f"     Registers: {', '.join(payload.get('registers', [])[:5])}")
        print(f"     Score: {hit['score']:.4f}")
    
    # Тест 3: DMA CCM RAM (должен найти f407_known_issues.md, но его нет)
    print("\n\n📝 Test 3: 'DMA CCM RAM incompatibility'")
    results = search("DMA CCM RAM incompatibility", limit=3)
    for i, hit in enumerate(results, 1):
        payload = hit["payload"]
        print(f"\n  {i}. {payload.get('title', 'N/A')}")
        print(f"     File: {payload.get('source_file', 'N/A')}")
        print(f"     Peripheral: {payload.get('peripheral', 'N/A')}")
        print(f"     Score: {hit['score']:.4f}")
    
    # Тест 4: Фильтр по peripheral=TIM
    print("\n\n📝 Test 4: 'PWM configuration' with filter peripheral=TIM")
    results = search("PWM configuration", limit=3, filters={
        "must": [{"key": "peripheral", "match": {"value": "TIM"}}]
    })
    for i, hit in enumerate(results, 1):
        payload = hit["payload"]
        print(f"\n  {i}. {payload.get('title', 'N/A')}")
        print(f"     File: {payload.get('source_file', 'N/A')}")
        print(f"     Peripheral: {payload.get('peripheral', 'N/A')}")
        print(f"     Score: {hit['score']:.4f}")
    
    # Статистика коллекции
    print("\n\n" + "=" * 60)
    print("📊 Collection Statistics")
    print("=" * 60)
    resp = requests.get(f"{QDRANT_URL}/collections/{COLLECTION}")
    stats = resp.json()["result"]
    print(f"  Status: {stats['status']}")
    print(f"  Points: {stats['points_count']}")
    print(f"  Vector size: {stats['config']['params']['vectors']['size']}")
    print(f"  Optimizer: {stats['optimizer_status']}")
    
    print("\n✅ Search test completed!")

if __name__ == "__main__":
    main()
