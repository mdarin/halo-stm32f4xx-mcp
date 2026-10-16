#!/usr/bin/env python3
"""
E2E: контекстная память агента (Фаза 5, CONTRACT.md §10).

Сценарий (стандартный STDIO MCP-клиент):
1. project_context_load — «агент открывает проект», пусто или старое значение
2. ctx-extract (toolkit) — периферия/IRQ из .ioc + AST
3. project_context_save — сводка проекта (summary) с peripherals/interrupts
4. project_context_load — «агент вспоминает»: TIM3 уже PWM
5. project_context_search — семантический поиск по сохранённому контексту
6. Сценарий предпочтений: save prefs → load prefs

Требует собранные mcp-server и toolkit (make build), живые Qdrant/Ollama.
Проект по умолчанию — shift-commander (gitignored подопытный).
"""
import faulthandler, json, os, subprocess, sys

faulthandler.dump_traceback_later(90, exit=True)

root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
server = os.environ.get("MCP_SERVER_PATH", os.path.join(root, "mcp-server", "mcp-server"))
toolkit_bin = os.environ.get("TOOLKIT_PATH", os.path.join(root, "toolkit", "target", "release", "toolkit"))
project = os.environ.get("E2E_PROJECT", os.path.join(os.path.dirname(root), "shift-commander"))

env = dict(os.environ, TOOLKIT_PATH=toolkit_bin)
p = subprocess.Popen([server], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                     stderr=subprocess.DEVNULL, env=env, text=True, cwd=root)

next_id = 1

def call(method, params=None, notify=False):
    global next_id
    msg = {"jsonrpc": "2.0", "method": method}
    if not notify:
        msg["id"] = next_id
        next_id += 1
    if params is not None:
        msg["params"] = params
    p.stdin.write(json.dumps(msg) + "\n")
    p.stdin.flush()
    if notify:
        return None
    while True:
        line = p.stdout.readline()
        if not line:
            raise RuntimeError("server closed stdout")
        try:
            resp = json.loads(line)
        except json.JSONDecodeError:
            continue
        if not notify and resp.get("id") == msg["id"]:
            if "error" in resp:
                raise RuntimeError(f"{method}: {resp['error']}")
            return resp.get("result", resp)

def tool(name, args):
    out = call("tools/call", {"name": name, "arguments": args})
    return json.loads(out["content"][0]["text"])

def fail(msg):
    print("E2E FAIL:", msg)
    p.terminate()
    sys.exit(1)

call("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                    "clientInfo": {"name": "e2e-context", "version": "0"}})
call("notifications/initialized", notify=True)

tl = call("tools/list")
if "result" in tl:
    tl = tl["result"]
names = [t["name"] for t in tl["tools"]]
for required in ("project_context_load", "project_context_save", "project_context_search"):
    if required not in names:
        fail(f"{required} not in tools/list")

# 1. Снимок «до»
before = tool("project_context_load", {"project_path": project})
print(f"1. load before: {before['count']} entries")

# 2. ctx-extract
r = subprocess.run([toolkit_bin, "ctx-extract", "--project", project],
                   capture_output=True, text=True, timeout=120)
if r.returncode != 0:
    fail(f"ctx-extract: {r.stderr}")
extracted = json.loads(r.stdout)
print(f"2. extract: {len(extracted['peripherals'])} peripherals, "
      f"{len(extracted['interrupts'])} IRQ, {len(extracted['blocks'])} blocks")

# 3. ctx-save summary
text = ("Project uses " + ", ".join(extracted["peripherals"]) +
        "; MCU: " + (extracted.get("mcu") or "unknown"))
saved = tool("project_context_save", {
    "project_path": project, "key": "summary", "text": text,
    "peripherals": extracted["peripherals"],
    "interrupts": extracted["interrupts"],
})
if saved["saved"] != 1:
    fail(f"ctx-save saved={saved['saved']}")
print(f"3. save summary: saved={saved['saved']} collection={saved['collection']}")

# 4. ctx-load — TIM3 виден
after = tool("project_context_load", {"project_path": project})
entry = next((e for e in after["entries"] if e["ctx_key"] == "summary"), None)
if entry is None:
    fail("summary entry not found after save")
if "TIM3" not in entry["peripherals"]:
    fail(f"TIM3 missing from saved peripherals: {entry['peripherals']}")
print(f"4. load after: {after['count']} entries, TIM3 present")

# 5. ctx-search
hits = tool("project_context_search", {
    "project_path": project, "query": "TIM3 PWM", "limit": 3})
if not hits["results"]:
    fail("ctx-search returned no results")
if hits["results"][0]["ctx_key"] != "summary":
    fail(f"unexpected top hit: {hits['results'][0]['ctx_key']}")
print(f"5. search: top={hits['results'][0]['ctx_key']} "
      f"score={hits['results'][0]['score']:.3f}")

# 6. prefs
tool("project_context_save", {
    "project_path": project, "key": "prefs", "doc_type": "preference",
    "text": "User prefers doxygen comments", "user_preferences": {"style": "doxygen"}})
prefs = tool("project_context_load", {"project_path": project, "key": "prefs"})
if prefs["count"] != 1 or prefs["entries"][0]["user_preferences"].get("style") != "doxygen":
    fail(f"prefs scenario failed: {prefs}")
print("6. prefs: style=doxygen round-trip OK")

p.terminate()
print("E2E CONTEXT OK")
