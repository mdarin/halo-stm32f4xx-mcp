#!/usr/bin/env python3
"""
Backup и восстановление Qdrant коллекции stm32f407_docs

Использование:
    # Backup
    python scripts/qdrant_backup.py backup
    
    # Restore
    python scripts/qdrant_backup.py restore
    
    # List backups
    python scripts/qdrant_backup.py list
"""

import json
import sys
import requests
import subprocess
from pathlib import Path
from datetime import datetime

QDRANT_URL = "http://localhost:6333"
COLLECTION = "stm32f407_docs"
BACKUP_DIR = Path("backups/qdrant")
MCP_SERVER_DIR = Path("mcp-server")


def get_mcp_version():
    """Получить версию MCP сервера из go.mod"""
    go_mod = MCP_SERVER_DIR / "go.mod"
    if not go_mod.exists():
        return "unknown"
    
    content = go_mod.read_text(encoding="utf-8")
    
    # Извлечь версию модуля
    for line in content.split("\n"):
        if line.startswith("module"):
            return line.split()[-1] if len(line.split()) > 1 else "unknown"
    
    return "unknown"


def get_mcp_sdk_version():
    """Получить версию MCP SDK из go.mod"""
    go_mod = MCP_SERVER_DIR / "go.mod"
    if not go_mod.exists():
        return "unknown"
    
    content = go_mod.read_text(encoding="utf-8")
    
    # Извлечь версию MCP SDK
    for line in content.split("\n"):
        if "modelcontextprotocol/go-sdk" in line and not line.strip().startswith("//"):
            parts = line.split()
            if len(parts) >= 2:
                return parts[-1]
    
    return "unknown"


def get_git_commit():
    """Получить текущий git commit hash"""
    try:
        result = subprocess.run(
            ["git", "rev-parse", "--short", "HEAD"],
            capture_output=True,
            text=True,
            cwd=".",
            timeout=5
        )
        if result.returncode == 0:
            return result.stdout.strip()
    except Exception:
        pass
    return "unknown"

def backup():
    """Экспорт коллекции в JSON файл"""
    print("=" * 60)
    print("📦 Qdrant Backup")
    print("=" * 60)
    
    # Проверка коллекции
    print(f"\n🔍 Checking collection '{COLLECTION}'...")
    resp = requests.get(f"{QDRANT_URL}/collections/{COLLECTION}")
    if resp.status_code != 200:
        print(f"❌ Collection not found: {resp.text}")
        return False
    
    info = resp.json()["result"]
    points_count = info["points_count"]
    print(f"  ✅ Found {points_count} points")
    
    # Создание директории backup
    BACKUP_DIR.mkdir(parents=True, exist_ok=True)
    
    # Имя файла с timestamp
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    backup_file = BACKUP_DIR / f"qdrant_backup_{timestamp}.json"
    
    # Экспорт всех точек
    print(f"\n📥 Exporting points...")
    all_points = []
    offset = None
    
    while True:
        payload = {
            "limit": 100,
            "with_payload": True,
            "with_vector": True
        }
        if offset:
            payload["offset"] = offset
        
        resp = requests.post(
            f"{QDRANT_URL}/collections/{COLLECTION}/points/scroll",
            json=payload
        )
        resp.raise_for_status()
        
        result = resp.json()["result"]
        points = result["points"]
        all_points.extend(points)
        
        print(f"\r  Exported {len(all_points)}/{points_count} points...", end="", flush=True)
        
        # Проверка наличия следующей страницы
        next_offset = result.get("next_page_offset")
        if not next_offset:
            break
        offset = next_offset
    
    print(f"\n  ✅ Exported {len(all_points)} points")
    
    # Сохранение в файл
    print(f"\n💾 Saving to {backup_file}...")
    
    # Получение версий для метаданных
    mcp_module = get_mcp_version()
    mcp_sdk = get_mcp_sdk_version()
    git_commit = get_git_commit()
    
    backup_data = {
        "collection": COLLECTION,
        "timestamp": timestamp,
        "points_count": len(all_points),
        "vector_size": info["config"]["params"]["vectors"]["size"],
        "mcp_server": {
            "module": mcp_module,
            "sdk_version": mcp_sdk,
            "git_commit": git_commit
        },
        "points": all_points
    }

    with open(backup_file, "w", encoding="utf-8") as f:
        json.dump(backup_data, f, indent=2, ensure_ascii=False)

    file_size_mb = backup_file.stat().st_size / (1024 * 1024)
    print(f"  ✅ Backup created: {backup_file.name} ({file_size_mb:.2f} MB)")

    print("\n" + "=" * 60)
    print("✅ Backup completed successfully!")
    print("=" * 60)
    print(f"\n📊 Statistics:")
    print(f"  Points: {len(all_points)}")
    print(f"  File: {backup_file}")
    print(f"  Size: {file_size_mb:.2f} MB")
    print(f"\n🔧 MCP Server versions:")
    print(f"  Module: {mcp_module}")
    print(f"  SDK: {mcp_sdk}")
    print(f"  Git commit: {git_commit}")
    
    return True


def restore(backup_file=None):
    """Импорт коллекции из JSON файла"""
    print("=" * 60)
    print("📦 Qdrant Restore")
    print("=" * 60)
    
    # Поиск последнего backup
    if not backup_file:
        print(f"\n🔍 Finding latest backup in {BACKUP_DIR}...")
        backups = sorted(BACKUP_DIR.glob("qdrant_backup_*.json"), reverse=True)
        if not backups:
            print(f"❌ No backups found in {BACKUP_DIR}")
            return False
        backup_file = backups[0]
        print(f"  ✅ Found: {backup_file.name}")
    else:
        backup_file = Path(backup_file)
        if not backup_file.exists():
            print(f"❌ Backup file not found: {backup_file}")
            return False
    
    # Загрузка backup
    print(f"\n📂 Loading backup...")
    with open(backup_file, "r", encoding="utf-8") as f:
        backup_data = json.load(f)
    
    points_count = backup_data["points_count"]
    vector_size = backup_data["vector_size"]
    points = backup_data["points"]
    print(f"  ✅ Loaded {points_count} points (vector size: {vector_size})")
    
    # Проверка существования коллекции
    print(f"\n🔍 Checking collection '{COLLECTION}'...")
    resp = requests.get(f"{QDRANT_URL}/collections")
    collections = resp.json()["result"]["collections"]
    exists = any(c["name"] == COLLECTION for c in collections)
    
    if exists:
        print(f"  ⚠️  Collection '{COLLECTION}' already exists")
        print(f"  Deleting...")
        resp = requests.delete(f"{QDRANT_URL}/collections/{COLLECTION}")
        resp.raise_for_status()
    
    # Создание коллекции
    print(f"  Creating collection '{COLLECTION}'...")
    resp = requests.put(f"{QDRANT_URL}/collections/{COLLECTION}", json={
        "vectors": {"size": vector_size, "distance": "Cosine"},
        "sparse_vectors": {"bm25": {}}
    })
    resp.raise_for_status()
    print(f"  ✅ Collection created")
    
    # Импорт точек
    print(f"\n📤 Importing points...")
    batch_size = 100
    for i in range(0, len(points), batch_size):
        batch = points[i:i+batch_size]
        resp = requests.put(
            f"{QDRANT_URL}/collections/{COLLECTION}/points",
            json={"points": batch}
        )
        resp.raise_for_status()
        print(f"\r  Imported {min(i+batch_size, len(points))}/{len(points)} points...", end="", flush=True)
    
    print(f"\n  ✅ Imported {len(points)} points")
    
    # Проверка
    resp = requests.get(f"{QDRANT_URL}/collections/{COLLECTION}")
    final_count = resp.json()["result"]["points_count"]
    
    print("\n" + "=" * 60)
    print("✅ Restore completed successfully!")
    print("=" * 60)
    print(f"\n📊 Statistics:")
    print(f"  Points restored: {final_count}")
    print(f"  Backup file: {backup_file.name}")
    
    return True


def list_backups():
    """Список всех backup файлов"""
    print("=" * 60)
    print("📦 Qdrant Backups")
    print("=" * 60)
    
    if not BACKUP_DIR.exists():
        print(f"\n❌ Backup directory not found: {BACKUP_DIR}")
        return
    
    backups = sorted(BACKUP_DIR.glob("qdrant_backup_*.json"), reverse=True)
    if not backups:
        print(f"\n❌ No backups found in {BACKUP_DIR}")
        return
    
    print(f"\n📁 Found {len(backups)} backup(s) in {BACKUP_DIR}:\n")
    
    for backup in backups:
        # Загрузка метаданных
        with open(backup, "r", encoding="utf-8") as f:
            data = json.load(f)

        timestamp = data.get("timestamp", "unknown")
        points_count = data.get("points_count", 0)
        file_size_mb = backup.stat().st_size / (1024 * 1024)
        
        # MCP Server версии (если есть)
        mcp_info = data.get("mcp_server", {})
        mcp_sdk = mcp_info.get("sdk_version", "N/A")
        git_commit = mcp_info.get("git_commit", "N/A")

        # Форматирование даты
        try:
            dt = datetime.strptime(timestamp, "%Y%m%d_%H%M%S")
            date_str = dt.strftime("%Y-%m-%d %H:%M:%S")
        except:
            date_str = timestamp

        print(f"  📦 {backup.name}")
        print(f"     Date: {date_str}")
        print(f"     Points: {points_count}")
        print(f"     Size: {file_size_mb:.2f} MB")
        print(f"     MCP SDK: {mcp_sdk}")
        print(f"     Git commit: {git_commit}")
        print()


def main():
    if len(sys.argv) < 2:
        print("Usage:")
        print("  python scripts/qdrant_backup.py backup    - Create backup")
        print("  python scripts/qdrant_backup.py restore   - Restore from latest backup")
        print("  python scripts/qdrant_backup.py list      - List all backups")
        print("  python scripts/qdrant_backup.py restore <file> - Restore from specific file")
        sys.exit(1)
    
    command = sys.argv[1]
    
    if command == "backup":
        success = backup()
    elif command == "restore":
        backup_file = sys.argv[2] if len(sys.argv) > 2 else None
        success = restore(backup_file)
    elif command == "list":
        success = list_backups()
    else:
        print(f"❌ Unknown command: {command}")
        sys.exit(1)
    
    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()
