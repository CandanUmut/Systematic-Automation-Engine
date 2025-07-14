Below is a clean “v-0.1.0” repo layout that swaps the lightweight SQLite layer for MongoDB (local or Atlas).  Every file is ready to paste into a folder and run.

agent-framework/
├─ README.md
├─ requirements.txt
├─ .env.example
├─ server.py              ← Flask API + Socket.IO + APScheduler
├─ db.py                  ← MongoDB persistence wrapper
├─ plugins/
│  ├─ __init__.py         ← auto-loader
│  └─ desktop.py          ← Windows GUI automation (pywinauto)
└─ static/                ← everything from your HTML/CSS/JS
   └─ index.html



⸻

1 requirements.txt

flask
flask_cors
flask_socketio
apscheduler
python-dotenv          # read .env
pymongo[srv]           # MongoDB driver
pywinauto==0.6.9       # desktop control
requests



⸻

2 .env.example

# copy to .env and edit
MONGO_URI=mongodb://localhost:27017
MONGO_DB=agent
OLLAMA_URL=http://localhost:11434
HOST=0.0.0.0
PORT=5000



⸻

3 db.py  (Mongo persistence layer)

# db.py
import os, json
from datetime import datetime
from pymongo import MongoClient
from dotenv import load_dotenv

load_dotenv()
client = MongoClient(os.getenv("MONGO_URI"))
db     = client[os.getenv("MONGO_DB", "agent")]
wfcol  = db.workflow
rcol   = db.run
jobcol = db.job

def now_iso():
    return datetime.utcnow().isoformat()

# ───── Workflows ───────────────────────────────────────────────
def save_workflow(wf:dict):
    wfcol.replace_one({"_id": wf["id"]}, wf, upsert=True)

def load_workflow(wf_id:str):
    return wfcol.find_one({"_id": wf_id})

def all_workflows():
    return list(wfcol.find({}, {"name":1,"created":1}))

# ───── Runs ────────────────────────────────────────────────────
def create_run(wf_id:str, status="running"):
    run_id = wf_id + "-" + now_iso()
    rcol.insert_one({
        "_id": run_id,
        "wf_id": wf_id,
        "status": status,
        "started": now_iso(),
        "logs": []
    })
    return run_id

def append_log(run_id, msg, level="info"):
    rcol.update_one({"_id": run_id},
        {"$push": {"logs": {"ts": now_iso(), "level": level, "msg": msg}}})

def finish_run(run_id, status="completed"):
    rcol.update_one({"_id": run_id},
        {"$set": {"status": status, "finished": now_iso()}})

# ───── Jobs (schedules) ────────────────────────────────────────
def save_job(job):
    jobcol.insert_one(job)

def upcoming_jobs():
    return list(jobcol.find({}))



⸻

4 plugins/init.py  (auto-loader)

import importlib, pkgutil

CAPABILITIES = {}

def register_capability(name:str, desc:str):
    def decorator(cls):
        CAPABILITIES[name] = {"class": cls, "desc": desc}
        return cls
    return decorator

def load_plugins():
    for _, mod_name, _ in pkgutil.iter_modules(__path__):
        importlib.import_module(f"{__name__}.{mod_name}")

load_plugins()               # run at import-time



⸻

5 plugins/desktop.py  (Windows GUI control)

# plugins/desktop.py
from pywinauto import Application
from . import register_capability

@register_capability("desktop-control",
                     desc="Click/type on Windows applications")
class DesktopControl:
    def __init__(self):
        self.app = None

    def execute(self, action:str, **kw):
        if action == "open":
            self.app = Application(backend="uia").start(kw["exe"])
        elif action in ("click", "type"):
            self.app = self.app or Application(backend="uia").connect(title_re=kw["window"])
            win = self.app.window(title_re=kw["window"])
            ctrl = win.child_window(title_re=kw["control"])
            if action == "click":
                ctrl.click_input()
            else:  # type
                ctrl.type_keys(kw["text"], with_spaces=True)
        return {"ok": True}



⸻

6 server.py  (core API)

#!/usr/bin/env python3
"""
AI-Agent scaffold backend (Mongo edition)
-----------------------------------------
• Serves /static on :5000
• Proxies Ollama on :11434
• Persists workflows + runs in MongoDB
"""
import os, json, time, uuid, asyncio
from threading import Thread
from dotenv import load_dotenv
from flask import Flask, request, jsonify, Response, send_from_directory, abort
from flask_cors import CORS
from flask_socketio import SocketIO, emit
from apscheduler.schedulers.background import BackgroundScheduler
import requests

import db                                   # your Mongo helpers
from plugins import CAPABILITIES            # auto-loaded

# ───── Config ──────────────────────────────────────────────────
load_dotenv()
OLLAMA_URL = os.getenv("OLLAMA_URL", "http://localhost:11434")
STATIC_DIR = "static"
HOST       = os.getenv("HOST", "0.0.0.0")
PORT       = int(os.getenv("PORT", "5000"))

app  = Flask(__name__, static_folder=STATIC_DIR, static_url_path="")
CORS(app, origins="*")
sio  = SocketIO(app, cors_allowed_origins="*", async_mode="threading")

aps  = BackgroundScheduler(); aps.start()

# ───── Helper: stream Ollama tokens → SSE ─────────────────────
def stream_sse(line_iter):
    for line in line_iter:
        yield f"data:{line.decode()}\n\n"

# ───── Static front-end ────────────────────────────────────────
@app.get("/")                       
def index():
    return send_from_directory(STATIC_DIR, "index.html")

@app.get("/<path:file>")
def assets(file):
    return send_from_directory(STATIC_DIR, file)

# ───── Ollama proxy ────────────────────────────────────────────
@app.post("/ollama/chat")
def ollama_chat():
    data = request.get_json(force=True)
    r = requests.post(f"{OLLAMA_URL}/api/chat", json=data, stream=data.get("stream",False))
    if data.get("stream"):
        return Response(stream_sse(r.iter_lines()), mimetype="text/event-stream")
    return (r.json(), r.status_code)

@app.get("/ollama/models")
def ollama_models():
    r = requests.get(f"{OLLAMA_URL}/api/tags")
    return (r.json(), r.status_code)

# ───── Capabilities list ───────────────────────────────────────
@app.get("/workflow/capabilities")
def list_caps():
    return jsonify({k:{"desc":v["desc"]} for k,v in CAPABILITIES.items()})

# ───── CRUD Workflows ──────────────────────────────────────────
@app.post("/workflow")
def create_wf():
    wf = request.get_json(force=True)
    wf["id"] = wf.get("id") or str(uuid.uuid4())
    wf["created"] = db.now_iso()
    db.save_workflow(wf)
    return jsonify({"id": wf["id"]})

@app.get("/workflow")
def list_wf():
    return jsonify(db.all_workflows())

@app.delete("/workflow/<wf_id>")
def delete_wf(wf_id):
    # easy: remove doc
    db.wfcol.delete_one({"_id": wf_id})
    return jsonify({"deleted": wf_id})

# ───── Run workflow ────────────────────────────────────────────
def render_placeholders(value, params):
    if isinstance(value,str):
        for k,v in params.items(): value = value.replace(f"{{{{{k}}}}}", str(v))
    return value

def execute_node(node, context):
    cap_name = node["capability"]
    cap_cls  = CAPABILITIES[cap_name]["class"]
    cap_obj  = context.setdefault(cap_name, cap_cls())
    return cap_obj.execute(**node)

def execute_workflow(wf, run_id, params):
    for raw in wf["nodes"]:
        node = {k:render_placeholders(v,params) for k,v in raw.items()}
        try:
            res = execute_node(node, context={})
            sio.emit("log", {"run": run_id, "msg": f"{node['capability']} ok"})
            db.append_log(run_id, f"{node['capability']} ok")
        except Exception as e:
            db.append_log(run_id, f"ERR {e}", level="error")
            sio.emit("log", {"run": run_id, "msg": str(e)})
            db.finish_run(run_id,"failed")
            return
    db.finish_run(run_id,"completed")
    sio.emit("log", {"run": run_id, "msg": "✅ completed"})

@app.post("/workflow/run/<wf_id>")
def run_wf(wf_id):
    wf   = db.load_workflow(wf_id) or abort(404,"workflow not found")
    params = request.get_json(silent=True) or {}
    run_id = db.create_run(wf_id)
    Thread(target=execute_workflow,args=(wf,run_id,params),daemon=True).start()
    return jsonify({"run_id": run_id})

# ───── Schedules (cron) ────────────────────────────────────────
@app.post("/workflow/schedule/<wf_id>")
def schedule_wf(wf_id):
    data = request.get_json(force=True)     # {"cron": "*/5 * * * *"}
    cron = data["cron"]
    # store in DB
    db.save_job({"wf_id": wf_id, "cron": cron})
    # schedule with APS
    aps.add_job(lambda: run_wf(wf_id), trigger="cron", **cron_to_kwargs(cron))
    return jsonify({"ok": True})

def cron_to_kwargs(expr):
    # very small 5-field parser
    minute,hour,dom,month,dow = expr.split()
    kw = {}
    if minute!="*": kw["minute"]=minute
    if hour!="*":   kw["hour"]=hour
    if dom!="*":    kw["day"]=dom
    if month!="*":  kw["month"]=month
    if dow!="*":    kw["day_of_week"]=dow
    return kw

# ───── WebSocket logging ───────────────────────────────────────
@sio.on("connect")
def ws_connect():
    emit("log",{"msg":"WS connected"})

# ───── Main ────────────────────────────────────────────────────
if __name__ == "__main__":
    print("✓ Plugins:",", ".join(CAPABILITIES))
    sio.run(app,host=HOST,port=PORT,debug=True)



⸻

7 static/index.html

Copy the HTML/CSS/JS you already built into static/index.html.
Because the REST endpoints (/workflow, /workflow/run, /workflow/capabilities, /ollama/*) are unchanged, the UI works untouched.
Add a small helper in your JS when you call run:

// example param-passing
fetch(`/workflow/run/${wfId}`, {
  method: 'POST',
  headers: {'Content-Type':'application/json'},
  body: JSON.stringify({title: 'My new video', description:'…'})
});



⸻

8 Running it

# 1. start MongoDB  (local or `docker run -p 27017:27017 mongo`)
cp .env.example .env           # edit if needed
pip install -r requirements.txt
python server.py               # backend + WebSocket
# 2. open http://localhost:5000 in your browser

You now have:
	•	Mongo-backed workflow persistence and run history
	•	Windows desktop control via desktop-control capability
	•	Endpoints your LLM agent can call with parameters to finish saved sequences

When you’re ready for the next layer (HTML report, React dashboard, AutoGen planner), ping me and we’ll stack it on top. Happy hacking! 🚀
