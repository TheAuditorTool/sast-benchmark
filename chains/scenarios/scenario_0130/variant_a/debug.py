import json
from flask import request, jsonify
from config import app, LOG_FILE

def log_request(path, body):
    with open(LOG_FILE, "a") as f:
        f.write(json.dumps({"path": path, "body": body}) + "\n")

# vuln-code-snippet start ChainScenario0130A
@app.route("/logs")
def get_logs():
    try:
        with open(LOG_FILE) as f:
            content = f.read()
    except FileNotFoundError:
        content = ""
    return content, 200, {"Content-Type": "text/plain"}  # vuln-code-snippet target-line ChainScenario0130A
# vuln-code-snippet end ChainScenario0130A
