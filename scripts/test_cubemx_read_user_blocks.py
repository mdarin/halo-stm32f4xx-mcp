#!/usr/bin/env python3
"""
Тест инструмента cubemx_read_user_blocks через STDIO protocol
"""
import subprocess
import json
import sys
import time

def send_request(proc, request):
    """Отправка JSON-RPC запроса и получение ответа"""
    json_line = json.dumps(request) + "\n"
    proc.stdin.write(json_line)
    proc.stdin.flush()
    
    response_line = proc.stdout.readline()
    if not response_line:
        return None
    
    return json.loads(response_line)

def test_read_single_file(proc):
    """Тест: чтение USER CODE блоков из одного файла"""
    print("\n📝 Test 1: Read USER CODE blocks from single file (main.c)")
    
    request = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "cubemx_read_user_blocks",
            "arguments": {
                "file_path": "C:/Users/mdari/projects/ShiftCommander/src/main.c"
            }
        }
    }
    
    response = send_request(proc, request)
    if response and "result" in response:
        content = response["result"].get("content", [])
        if content and content[0].get("type") == "text":
            result_text = content[0]["text"]
            result_data = json.loads(result_text)
            
            print(f"  ✅ Read completed")
            print(f"     Files: {result_data.get('files_count')}")
            print(f"     Total blocks: {result_data.get('total_blocks')}")
            
            files = result_data.get("files", [])
            if files:
                blocks = files[0].get("blocks", [])
                print(f"     Blocks in main.c: {len(blocks)}")
                
                # Показываем первые 5 блоков
                for i, block in enumerate(blocks[:5], 1):
                    name = block.get("name", "N/A")
                    line = block.get("line", 0)
                    content_preview = block.get("content", "")[:50]
                    print(f"       {i}. Block '{name}' (line {line}): {content_preview}...")
                
                return len(blocks) > 0
    
    print(f"  ❌ Test failed: {response}")
    return False

def test_read_project(proc):
    """Тест: чтение USER CODE блоков из всего проекта"""
    print("\n📝 Test 2: Read USER CODE blocks from entire project")
    
    request = {
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {
            "name": "cubemx_read_user_blocks",
            "arguments": {
                "project_root": "C:/Users/mdari/projects/ShiftCommander"
            }
        }
    }
    
    response = send_request(proc, request)
    if response and "result" in response:
        content = response["result"].get("content", [])
        if content and content[0].get("type") == "text":
            result_text = content[0]["text"]
            result_data = json.loads(result_text)
            
            print(f"  ✅ Read completed")
            print(f"     Files: {result_data.get('files_count')}")
            print(f"     Total blocks: {result_data.get('total_blocks')}")
            
            files = result_data.get("files", [])
            print(f"\n     Files with USER CODE blocks:")
            for i, f in enumerate(files[:10], 1):
                file_path = f.get("file_path", "N/A")
                blocks_count = f.get("blocks_count", 0)
                # Показываем только имя файла, не полный путь
                file_name = file_path.split("/")[-1] if "/" in file_path else file_path
                print(f"       {i}. {file_name}: {blocks_count} blocks")
            
            return result_data.get("total_blocks", 0) > 0
    
    print(f"  ❌ Test failed: {response}")
    return False

def main():
    print("=" * 60)
    print("🧪 cubemx_read_user_blocks Tool Test")
    print("=" * 60)
    
    # Запуск MCP сервера
    print("\n🚀 Starting MCP server...")
    proc = subprocess.Popen(
        ["mcp-server/mcp-server.exe"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        encoding="utf-8"
    )
    
    time.sleep(1)
    
    if proc.poll() is not None:
        print(f"❌ Server exited immediately with code {proc.returncode}")
        stderr = proc.stderr.read()
        print(f"Stderr: {stderr}")
        return False
    
    print("  ✅ Server started")
    
    try:
        # Инициализация MCP сессии
        print("\n🔌 Initializing MCP session...")
        init_request = {
            "jsonrpc": "2.0",
            "id": 0,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {
                    "name": "test-client",
                    "version": "0.1.0"
                }
            }
        }
        init_response = send_request(proc, init_request)
        if not init_response or "error" in init_response:
            print(f"  ❌ Initialization failed: {init_response}")
            return False
        print(f"  ✅ Session initialized")
        
        # Отправка initialized notification
        initialized_notification = {
            "jsonrpc": "2.0",
            "method": "notifications/initialized"
        }
        proc.stdin.write(json.dumps(initialized_notification) + "\n")
        proc.stdin.flush()
        time.sleep(0.5)
        
        # Тесты
        results = []
        results.append(("Read single file", test_read_single_file(proc)))
        results.append(("Read entire project", test_read_project(proc)))
        
        # Итог
        print("\n" + "=" * 60)
        print("📊 Test Results")
        print("=" * 60)
        
        passed = sum(1 for _, success in results if success)
        total = len(results)
        
        for name, success in results:
            status = "✅ PASS" if success else "❌ FAIL"
            print(f"  {status}: {name}")
        
        print(f"\n  Total: {passed}/{total} tests passed")
        
        if passed == total:
            print("\n✅ All tests passed!")
            return True
        else:
            print(f"\n❌ {total - passed} test(s) failed")
            return False
    
    finally:
        proc.terminate()
        proc.wait(timeout=5)
        print("\n🛑 Server stopped")

if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)
