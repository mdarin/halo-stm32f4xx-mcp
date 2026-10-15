#!/usr/bin/env python3
"""
Тест инструмента check_f407_rules через STDIO protocol
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

def test_check_ccm_dma(proc):
    """Тест: DMA буфер в CCM RAM"""
    print("\n📝 Test 1: CCM DMA incompatibility")
    
    code = """
uint8_t dma_buffer[1024] __attribute__((section(".ccmram")));
"""
    
    request = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "check_f407_rules",
            "arguments": {
                "code": code,
                "file_path": "main.c"
            }
        }
    }
    
    response = send_request(proc, request)
    if response and "result" in response:
        content = response["result"].get("content", [])
        if content and content[0].get("type") == "text":
            result_text = content[0]["text"]
            result_data = json.loads(result_text)
            
            print(f"  ✅ Check completed")
            print(f"     Violations: {result_data.get('violations_count')}")
            
            violations = result_data.get("violations", [])
            for v in violations:
                print(f"     - [{v['severity']}] {v['rule']}: {v['message'][:60]}...")
            
            # Проверка, что найдено нарушение ccm_dma_incompatible
            ccm_found = any(v["rule"] == "ccm_dma_incompatible" for v in violations)
            if ccm_found:
                print(f"     ✅ CCM DMA violation detected!")
                return True
            else:
                print(f"     ❌ CCM DMA violation NOT detected")
                return False
    
    print(f"  ❌ Test failed: {response}")
    return False

def test_check_nvic_priority(proc):
    """Тест: NVIC priority grouping"""
    print("\n📝 Test 2: NVIC priority grouping")
    
    code = """
HAL_NVIC_SetPriorityGrouping(NVIC_PRIORITYGROUP_3);
"""
    
    request = {
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {
            "name": "check_f407_rules",
            "arguments": {
                "code": code,
                "file_path": "main.c"
            }
        }
    }
    
    response = send_request(proc, request)
    if response and "result" in response:
        content = response["result"].get("content", [])
        if content and content[0].get("type") == "text":
            result_text = content[0]["text"]
            result_data = json.loads(result_text)
            
            print(f"  ✅ Check completed")
            print(f"     Violations: {result_data.get('violations_count')}")
            
            violations = result_data.get("violations", [])
            for v in violations:
                print(f"     - [{v['severity']}] {v['rule']}: {v['message'][:60]}...")
            
            nvic_found = any(v["rule"] == "nvic_priority_grouping" for v in violations)
            if nvic_found:
                print(f"     ✅ NVIC priority violation detected!")
                return True
            else:
                print(f"     ❌ NVIC priority violation NOT detected")
                return False
    
    print(f"  ❌ Test failed: {response}")
    return False

def test_check_usb_48mhz(proc):
    """Тест: USB без 48 MHz"""
    print("\n📝 Test 3: USB 48MHz requirement")
    
    code = """
HAL_PCD_Init(&hpcd_USB_OTG_FS);
"""
    
    request = {
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "check_f407_rules",
            "arguments": {
                "code": code,
                "file_path": "usb.c"
            }
        }
    }
    
    response = send_request(proc, request)
    if response and "result" in response:
        content = response["result"].get("content", [])
        if content and content[0].get("type") == "text":
            result_text = content[0]["text"]
            result_data = json.loads(result_text)
            
            print(f"  ✅ Check completed")
            print(f"     Violations: {result_data.get('violations_count')}")
            
            violations = result_data.get("violations", [])
            for v in violations:
                print(f"     - [{v['severity']}] {v['rule']}: {v['message'][:60]}...")
            
            usb_found = any(v["rule"] == "usb_48mhz_required" for v in violations)
            if usb_found:
                print(f"     ✅ USB 48MHz violation detected!")
                return True
            else:
                print(f"     ❌ USB 48MHz violation NOT detected")
                return False
    
    print(f"  ❌ Test failed: {response}")
    return False

def test_check_clean_code(proc):
    """Тест: чистый код без нарушений"""
    print("\n📝 Test 4: Clean code (no violations)")
    
    code = """
uint8_t dma_buffer[1024];  // SRAM1 by default
HAL_NVIC_SetPriorityGrouping(NVIC_PRIORITYGROUP_4);
"""
    
    request = {
        "jsonrpc": "2.0",
        "id": 4,
        "method": "tools/call",
        "params": {
            "name": "check_f407_rules",
            "arguments": {
                "code": code,
                "file_path": "main.c"
            }
        }
    }
    
    response = send_request(proc, request)
    if response and "result" in response:
        content = response["result"].get("content", [])
        if content and content[0].get("type") == "text":
            result_text = content[0]["text"]
            result_data = json.loads(result_text)
            
            print(f"  ✅ Check completed")
            print(f"     Violations: {result_data.get('violations_count')}")
            
            if result_data.get("violations_count") == 0:
                print(f"     ✅ No violations detected (as expected)")
                return True
            else:
                print(f"     ❌ Unexpected violations found")
                return False
    
    print(f"  ❌ Test failed: {response}")
    return False

def main():
    print("=" * 60)
    print("🧪 check_f407_rules Tool Test")
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
        results.append(("CCM DMA", test_check_ccm_dma(proc)))
        results.append(("NVIC Priority", test_check_nvic_priority(proc)))
        results.append(("USB 48MHz", test_check_usb_48mhz(proc)))
        results.append(("Clean Code", test_check_clean_code(proc)))
        
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
