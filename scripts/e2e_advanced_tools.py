#!/usr/bin/env python3
"""
E2E smoke: STDIO MCP-клиент -> advanced tools (ast_analyze_code,
advanced_lint flow/no-flow, generate_doc_comments) + ошибка на пустых аргументах.
Требует собранный mcp-server и toolkit (make build), TOOLKIT_PATH опционален
(по умолчанию toolkit/target/release/toolkit).
"""
import json, subprocess, os, sys

root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
server = os.environ.get("MCP_SERVER_PATH", os.path.join(root, "mcp-server", "mcp-server"))
toolkit_bin = os.environ.get("TOOLKIT_PATH", os.path.join(root, "toolkit", "target", "release", "toolkit"))

env = dict(os.environ, TOOLKIT_PATH=toolkit_bin)
p = subprocess.Popen([server], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, env=env, text=True, cwd=root)

def send(obj):
    p.stdin.write(json.dumps(obj) + "\n"); p.stdin.flush()

def recv():
    line = p.stdout.readline()
    return json.loads(line)

send({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"smoke","version":"0"}}})
init = recv()
send({"jsonrpc":"2.0","method":"notifications/initialized"})
send({"jsonrpc":"2.0","id":2,"method":"tools/list"})
tools = recv()
names = [t["name"] for t in tools["result"]["tools"]]
print("TOOLS:", names)

code = 'static uint8_t __attribute__((section(".ccmram"))) adc_dma_buffer[256];\nvoid start_adc_dma(void)\n{\n    HAL_ADC_Start_DMA(&hadc1, adc_dma_buffer, 256);\n}\n'

failures = []

tests = [
    ("tools/call", 3, {"name":"ast_analyze_code","arguments":{"code":code}}),
    ("tools/call", 4, {"name":"advanced_lint","arguments":{"code":code}}),
    ("tools/call", 5, {"name":"advanced_lint","arguments":{"code":code,"flow":True}}),
    ("tools/call", 6, {"name":"generate_doc_comments","arguments":{"code":"static void helper(int x)\n{\n    (void)x;\n}\n"}}),
    ("tools/call", 7, {"name":"advanced_lint","arguments":{}}),
]
for method, i, params in tests:
    send({"jsonrpc":"2.0","id":i,"method":method,"params":params})
    r = recv()
    res = r.get("result", {})
    if "result" in r and "content" in res and not res.get("isError"):
        txt = res["content"][0]["text"]
        d = json.loads(txt)
        if "violations" in d:
            print(f"[{i}] advanced_lint: violations={len(d['violations'])} summary={d.get('summary')} flows={'flows' in d and d['flows'] is not None}")
            for v in d['violations']: print("   ", v['rule'], v['severity'], f"line {v['position']['line']}")
            if i == 4 and not d['violations']:
                failures.append("[4] expected dma_buffer_in_ccmram violation")
        elif "insertions" in d:
            print(f"[{i}] generate_doc_comments: documented={d['functions_documented']} skipped={d['functions_skipped']}")
            print("   ", repr(d['insertions'][0]['text'][:60]) if d['insertions'] else "none")
        else:
            print(f"[{i}] ast_analyze: functions={[f['name'] for f in d['functions']]} hal_calls={[c['callee'] for c in d['hal_calls']]} variables={[v['name'] for v in d['variables']]} parse_errors={len(d['parse_errors'])}")
            if [f['name'] for f in d['functions']] != ['start_adc_dma'] or [c['callee'] for c in d['hal_calls']] != ['HAL_ADC_Start_DMA']:
                failures.append(f"[{i}] unexpected analyze output")
    else:
        print(f"[{i}] ERROR (expected for empty args):", str(r.get("error") or res.get("content"))[:150])
p.terminate()

print("=" * 50)
if failures:
    print(f"FAIL: {len(failures)} check(s):")
    for f in failures: print("  -", f)
    sys.exit(1)
print("E2E advanced tools: all checks passed")
