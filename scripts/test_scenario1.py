#!/usr/bin/env python3
"""
Сценарий 1: Тест валидации проекта ShiftCommander
"""
import subprocess
import json
import time
import sys
import threading
import queue

def send(proc, data):
    proc.stdin.write(json.dumps(data) + "\n")
    proc.stdin.flush()

def reader_thread(proc, q):
    """Поток для чтения ответов"""
    while True:
        line = proc.stdout.readline()
        if not line:
            break
        q.put(json.loads(line))

# Запуск MCP сервера
proc = subprocess.Popen(
    ["mcp-server/mcp-server.exe"],
    stdin=subprocess.PIPE,
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
    text=True,
    encoding="utf-8"
)

# Запуск потока чтения
q = queue.Queue()
thread = threading.Thread(target=reader_thread, args=(proc, q), daemon=True)
thread.start()

time.sleep(1)

# Initialize
send(proc, {"jsonrpc":"2.0","id":0,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"0.1.0"}}})
try:
    resp = q.get(timeout=5)
    print(f"✅ Initialize: {resp.get('result',{}).get('serverInfo',{})}")
except queue.Empty:
    print("❌ Initialize timeout")
    sys.exit(1)

# Initialized notification
send(proc, {"jsonrpc":"2.0","method":"notifications/initialized"})
time.sleep(0.5)

# Test cubemx_validate_project
print("\n--- cubemx_validate_project ---")
send(proc, {"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"cubemx_validate_project","arguments":{"project_root":"C:/Users/mdari/projects/ShiftCommander"}}})
try:
    resp = q.get(timeout=30)
    if resp and "result" in resp:
        content = resp["result"]["content"][0]["text"]
        data = json.loads(content)
        print(f"✅ Valid: {data['valid']}")
        print(f"✅ Files: {data['files_count']}")
        print(f"✅ Blocks: {data['blocks_count']}")
        print(f"✅ CubeMX: {data['cubemx_version']}")
        if data['errors']:
            print(f"❌ Errors: {data['errors']}")
        if data['warnings']:
            print(f"⚠️  Warnings: {data['warnings']}")
        print("\n✅ Сценарий 1 пройден!")
    else:
        print(f"❌ Error: {resp}")
except queue.Empty:
    print("❌ Timeout waiting for response")

proc.terminate()
