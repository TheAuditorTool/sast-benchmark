import json
from flask import request, jsonify
from module_b import app, LOG_FILE

def log_request(path, body):
    with open(LOG_FILE, "a") as f:
        f.write(json.dumps({"path": path, "body": body}) + "\n")

@app.route("/logs")
def get_logs():
    try:
        with open(LOG_FILE) as f:
            content = f.read()
    except FileNotFoundError:
        content = ""
    return content, 200, {"Content-Type": "text/plain"}
