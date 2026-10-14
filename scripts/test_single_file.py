#!/usr/bin/env python3
import json, requests
from pathlib import Path

KB_DIR = Path("kb")
manifest = json.loads((KB_DIR / "manifest.json").read_text(encoding="utf-8"))

# Взять первый файл
f = manifest["files"][0]
print(f"Testing with: {f['file']}")

content = (KB_DIR / f["file"]).read_text(encoding="utf-8")
print(f"Content length: {len(content)} chars")

# Эмбеддинг (первые 500 символов)
print("Requesting embedding from Ollama...")
resp = requests.post("http://localhost:11434/api/embed", 
                    json={"model": "nomic-embed-text", "input": content[:500]},
                    timeout=60)
print(f"Embed status: {resp.status_code}")
embedding = resp.json()["embeddings"][0]
print(f"Embedding dims: {len(embedding)}")

# Загрузка в Qdrant
print("Uploading to Qdrant...")
point = {
    "id": 12345,
    "vector": embedding,
    "payload": {"text": content[:500], "source_file": f["file"]}
}
resp = requests.put("http://localhost:6333/collections/stm32f407_docs/points",
                   json={"points": [point]})
print(f"Qdrant upload status: {resp.status_code}")
print(f"Response: {resp.text[:200]}")

# Проверка количества точек
resp = requests.get("http://localhost:6333/collections/stm32f407_docs")
print(f"Points in collection: {resp.json()['result']['points_count']}")
