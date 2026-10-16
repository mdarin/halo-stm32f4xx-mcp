#!/usr/bin/env python3
"""
Тест MCP сервера через STDIO protocol
"""
import subprocess
import os
import json
import sys
import time

def send_request(proc, request):
    """Отправка JSON-RPC запроса и получение ответа"""
    json_line = json.dumps(request) + "\n"
    proc.stdin.write(json_line)
    proc.stdin.flush()
    
    # Чтение ответа
    response_line = proc.stdout.readline()
    if not response_line:
        return None
    
    return json.loads(response_line)

def test_initialize(proc):
    """Тест метода initialize"""
    print("\n📝 Test 1: initialize")
    request = {
        "jsonrpc": "2.0",
        "id": 1,
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
    
    response = send_request(proc, request)
    if response and "result" in response:
        print(f"  ✅ Initialize successful")
        print(f"     Server: {response['result'].get('serverInfo', {}).get('name')}")
        print(f"     Version: {response['result'].get('serverInfo', {}).get('version')}")
        return True
    else:
        print(f"  ❌ Initialize failed: {response}")
        return False

def test_tools_list(proc):
    """Тест метода tools/list"""
    print("\n📝 Test 2: tools/list")
    request = {
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list",
        "params": {}
    }
    
    response = send_request(proc, request)
    if response and "result" in response:
        tools = response["result"].get("tools", [])
        print(f"  ✅ Found {len(tools)} tool(s)")
        for tool in tools:
            print(f"     - {tool['name']}: {tool.get('description', 'N/A')[:50]}...")
        return len(tools) > 0
    else:
        print(f"  ❌ tools/list failed: {response}")
        return False

def test_query_hardware_docs(proc):
    """Тест инструмента query_hardware_docs"""
    print("\n📝 Test 3: query_hardware_docs (GPIO MODER)")
    request = {
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "query_hardware_docs",
            "arguments": {
                "query": "GPIO MODER register configuration",
                "limit": 3
            }
        }
    }
    
    response = send_request(proc, request)
    if response and "result" in response:
        content = response["result"].get("content", [])
        if content and content[0].get("type") == "text":
            result_text = content[0]["text"]
            result_data = json.loads(result_text)
            
            print(f"  ✅ Search successful")
            print(f"     Query: {result_data.get('query')}")
            print(f"     Count: {result_data.get('count')}")
            
            results = result_data.get("results", [])
            for i, hit in enumerate(results[:3], 1):
                print(f"     {i}. {hit.get('title', 'N/A')} (score: {hit.get('score', 0):.4f})")
                print(f"        File: {hit.get('source', 'N/A')}")
            
            return len(results) > 0
        else:
            print(f"  ❌ Unexpected content format: {content}")
            return False
    else:
        print(f"  ❌ tools/call failed: {response}")
        return False

def test_query_with_filter(proc):
    """Тест инструмента query_hardware_docs с фильтром peripheral"""
    print("\n📝 Test 4: query_hardware_docs with peripheral=TIM")
    request = {
        "jsonrpc": "2.0",
        "id": 4,
        "method": "tools/call",
        "params": {
            "name": "query_hardware_docs",
            "arguments": {
                "query": "PWM configuration",
                "peripheral": "TIM",
                "limit": 3
            }
        }
    }
    
    response = send_request(proc, request)
    if response and "result" in response:
        content = response["result"].get("content", [])
        if content and content[0].get("type") == "text":
            result_text = content[0]["text"]
            result_data = json.loads(result_text)
            
            print(f"  ✅ Search with filter successful")
            print(f"     Query: {result_data.get('query')}")
            print(f"     Peripheral: {result_data.get('peripheral')}")
            print(f"     Count: {result_data.get('count')}")
            
            results = result_data.get("results", [])
            for i, hit in enumerate(results[:3], 1):
                print(f"     {i}. {hit.get('title', 'N/A')} (score: {hit.get('score', 0):.4f})")
                print(f"        Peripheral: {hit.get('peripheral', 'N/A')}")
            
            # Проверка, что все результаты имеют peripheral=TIM
            all_tim = all(hit.get("peripheral") == "TIM" for hit in results)
            if all_tim:
                print(f"     ✅ All results have peripheral=TIM")
            else:
                print(f"     ⚠️  Some results don't have peripheral=TIM")
            
            return len(results) > 0
        else:
            print(f"  ❌ Unexpected content format: {content}")
            return False
    else:
        print(f"  ❌ tools/call failed: {response}")
        return False

def main():
    print("=" * 60)
    print("🧪 MCP Server STDIO Test")
    print("=" * 60)
    
    # Запуск MCP сервера
    print("\n🚀 Starting MCP server...")
    proc = subprocess.Popen(
        [os.environ.get("MCP_SERVER_PATH", "mcp-server/mcp-server")],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        encoding="utf-8"
    )
    
    # Дать серверу время на запуск
    time.sleep(1)
    
    # Проверка, что сервер запущен
    if proc.poll() is not None:
        print(f"❌ Server exited immediately with code {proc.returncode}")
        stderr = proc.stderr.read()
        print(f"Stderr: {stderr}")
        return False
    
    print("  ✅ Server started")
    
    try:
        # Тесты
        results = []
        results.append(("initialize", test_initialize(proc)))
        results.append(("tools/list", test_tools_list(proc)))
        results.append(("query_hardware_docs", test_query_hardware_docs(proc)))
        results.append(("query with filter", test_query_with_filter(proc)))
        
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
        # Остановка сервера
        proc.terminate()
        proc.wait(timeout=5)
        print("\n🛑 Server stopped")

if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)
