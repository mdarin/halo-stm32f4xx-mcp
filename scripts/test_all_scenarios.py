#!/usr/bin/env python3
"""
Комплексный тест всех 7 сценариев для MCP сервера stm32f407
"""
import subprocess
import json
import time
import sys
import threading
import queue

PROJECT_ROOT = "C:/Users/mdari/projects/ShiftCommander"
MAIN_C = "C:/Users/mdari/projects/ShiftCommander/src/main.c"

def send(proc, data):
    proc.stdin.write(json.dumps(data) + "\n")
    proc.stdin.flush()

def reader_thread(proc, q):
    while True:
        line = proc.stdout.readline()
        if not line:
            break
        try:
            q.put(json.loads(line))
        except:
            pass

def call_tool(proc, q, tool_name, args, timeout=30):
    send(proc, {"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":tool_name,"arguments":args}})
    try:
        resp = q.get(timeout=timeout)
        if resp and "result" in resp:
            content = resp["result"].get("content", [])
            if content and content[0].get("type") == "text":
                text = content[0].get("text", "")
                if text:
                    # Попытка парсинга JSON
                    try:
                        return json.loads(text)
                    except json.JSONDecodeError as e:
                        # Если не JSON, возвращаем как есть (для отладки)
                        print(f"  ⚠️  Response is not JSON: {text[:200]}...")
                        return {"raw_text": text}
                else:
                    print(f"  ⚠️  Empty text in response")
                    return None
            else:
                print(f"  ⚠️  Unexpected content format: {content}")
                return None
        elif resp and "error" in resp:
            print(f"  ❌ Error: {resp['error'].get('message', 'unknown')}")
            return None
        return resp
    except queue.Empty:
        print(f"  ⚠️  Timeout waiting for response")
        return None

# Запуск MCP сервера
print("=" * 70)
print("🧪 Комплексный тест MCP сервера stm32f407 (все сценарии)")
print("=" * 70)

proc = subprocess.Popen(
    ["mcp-server/mcp-server.exe"],
    stdin=subprocess.PIPE,
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
    text=True,
    encoding="utf-8"
)

q = queue.Queue()
thread = threading.Thread(target=reader_thread, args=(proc, q), daemon=True)
thread.start()
time.sleep(1)

# Initialize
send(proc, {"jsonrpc":"2.0","id":0,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"0.1.0"}}})
try:
    resp = q.get(timeout=5)
    print(f"\n✅ MCP Server: {resp.get('result',{}).get('serverInfo',{})}")
except queue.Empty:
    print("❌ Initialize timeout")
    sys.exit(1)

send(proc, {"jsonrpc":"2.0","method":"notifications/initialized"})
time.sleep(0.5)

results = []

# ============================================================================
# Сценарий 1: cubemx_validate_project
# ============================================================================
print("\n" + "=" * 70)
print("📝 Сценарий 1: cubemx_validate_project")
print("=" * 70)
data = call_tool(proc, q, "cubemx_validate_project", {"project_root": PROJECT_ROOT})
if data:
    print(f"  Valid: {data['valid']}")
    print(f"  Files: {data['files_count']}")
    print(f"  Blocks: {data['blocks_count']}")
    print(f"  Errors: {len(data['errors'])}")
    print(f"  Warnings: {len(data['warnings'])}")
    results.append(("validate_project", data['valid'] and data['files_count'] > 0))
else:
    print("  ❌ Failed")
    results.append(("validate_project", False))

# ============================================================================
# Сценарий 2: cubemx_read_user_blocks (один файл)
# ============================================================================
print("\n" + "=" * 70)
print("📝 Сценарий 2: cubemx_read_user_blocks (main.c)")
print("=" * 70)
data = call_tool(proc, q, "cubemx_read_user_blocks", {"file_path": MAIN_C})
if data:
    print(f"  Files: {data['files_count']}")
    print(f"  Total blocks: {data['total_blocks']}")
    if data['files']:
        blocks = data['files'][0]['blocks']
        print(f"  Blocks in main.c: {len(blocks)}")
        print(f"  First 5 blocks: {[b['name'] for b in blocks[:5]]}")
    results.append(("read_user_blocks", data['total_blocks'] > 0))
else:
    print("  ❌ Failed")
    results.append(("read_user_blocks", False))

# ============================================================================
# Сценарий 3: cubemx_find_weak_overrides
# ============================================================================
print("\n" + "=" * 70)
print("📝 Сценарий 3: cubemx_find_weak_overrides")
print("=" * 70)
data = call_tool(proc, q, "cubemx_find_weak_overrides", {"project_root": PROJECT_ROOT})
if data:
    print(f"  Overrides found: {data['count']}")
    for o in data['overrides'][:5]:
        print(f"    - {o['function']} in {o['file'].split('/')[-1]}:{o['line']}")
    results.append(("find_weak_overrides", data['count'] > 0))
else:
    print("  ❌ Failed")
    results.append(("find_weak_overrides", False))

# ============================================================================
# Сценарий 4: query_hardware_docs (GPIO)
# ============================================================================
print("\n" + "=" * 70)
print("📝 Сценарий 4: query_hardware_docs (GPIO MODER)")
print("=" * 70)
data = call_tool(proc, q, "query_hardware_docs", {"query": "GPIO MODER register configuration", "limit": 3})
if data:
    print(f"  Query: {data['query']}")
    print(f"  Results: {data['count']}")
    for r in data['results'][:3]:
        print(f"    - {r.get('title','N/A')} (score: {r.get('score',0):.4f})")
        print(f"      Source: {r.get('source','N/A')}")
    results.append(("query_hardware_docs", data['count'] > 0 and data['results'][0]['score'] > 0.5))
else:
    print("  ❌ Failed")
    results.append(("query_hardware_docs", False))

# ============================================================================
# Сценарий 5: check_f407_rules (CCM DMA)
# ============================================================================
print("\n" + "=" * 70)
print("📝 Сценарий 5: check_f407_rules (CCM DMA violation)")
print("=" * 70)
test_code = 'uint8_t dma_buffer[1024] __attribute__((section(".ccmram")));'
data = call_tool(proc, q, "check_f407_rules", {"code": test_code, "file_path": "test.c"})
if data:
    print(f"  Violations: {data['violations_count']}")
    for v in data['violations']:
        print(f"    - [{v['severity']}] {v['rule']}: {v['message'][:60]}...")
    results.append(("check_f407_rules", data['violations_count'] > 0 and any(v['rule'] == 'ccm_dma_incompatible' for v in data['violations'])))
else:
    print("  ❌ Failed")
    results.append(("check_f407_rules", False))

# ============================================================================
# Сценарий 6: cubemx_insert_code (тест на временном файле)
# ============================================================================
print("\n" + "=" * 70)
print("📝 Сценарий 6: cubemx_insert_code (append mode)")
print("=" * 70)
# Создаём временный файл с USER CODE блоком
import tempfile
import os
temp_file = os.path.join(tempfile.gettempdir(), "test_cubemx.c")
with open(temp_file, "w") as f:
    f.write("""
/* USER CODE BEGIN Includes */
/* USER CODE END Includes */

/* USER CODE BEGIN 2 */
/* USER CODE END 2 */
""")

data = call_tool(proc, q, "cubemx_insert_code", {
    "file_path": temp_file,
    "block_name": "Includes",
    "code": '#include "test.h"',
    "mode": "append"
})
if data and data.get('success'):
    print(f"  Success: {data['success']}")
    print(f"  Block: {data['block_name']}")
    print(f"  Bytes: {data['bytes_written']}")
    # Проверяем, что код действительно вставлен
    with open(temp_file, "r") as f:
        content = f.read()
        if '#include "test.h"' in content:
            print(f"  ✅ Code inserted correctly")
            results.append(("insert_code", True))
        else:
            print(f"  ❌ Code not found in file")
            results.append(("insert_code", False))
    os.remove(temp_file)
else:
    print(f"  ❌ Failed: {data}")
    results.append(("insert_code", False))

# ============================================================================
# Сценарий 7: query_hardware_docs с фильтром peripheral
# ============================================================================
print("\n" + "=" * 70)
print("📝 Сценарий 7: query_hardware_docs (TIM + filter)")
print("=" * 70)
data = call_tool(proc, q, "query_hardware_docs", {"query": "PWM configuration", "peripheral": "TIM", "limit": 3})
if data:
    print(f"  Query: {data['query']}")
    print(f"  Peripheral: {data['peripheral']}")
    print(f"  Results: {data['count']}")
    all_tim = all(r.get('peripheral') == 'TIM' for r in data['results'])
    print(f"  All results have peripheral=TIM: {all_tim}")
    results.append(("query_with_filter", data['count'] > 0 and all_tim))
else:
    print("  ❌ Failed")
    results.append(("query_with_filter", False))

# ============================================================================
# Итог
# ============================================================================
print("\n" + "=" * 70)
print("📊 Итоговые результаты")
print("=" * 70)

passed = sum(1 for _, success in results if success)
total = len(results)

for name, success in results:
    status = "✅ PASS" if success else "❌ FAIL"
    print(f"  {status}: {name}")

print(f"\n  Total: {passed}/{total} tests passed")

if passed == total:
    print("\n🎉 Все сценарии пройдены!")
    exit_code = 0
else:
    print(f"\n❌ {total - passed} test(s) failed")
    exit_code = 1

proc.terminate()
sys.exit(exit_code)
