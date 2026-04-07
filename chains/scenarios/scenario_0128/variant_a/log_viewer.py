import os
from flask import Blueprint, request, jsonify
import config

log_viewer_bp = Blueprint("log_viewer", __name__)

@log_viewer_bp.route("/logs")
def view_log():
    log_name = request.args.get("name", "app.log")
    log_path = os.path.join(config.LOG_DIR, log_name)
# vuln-code-snippet start ChainScenario0128A
    with open(log_path, "r") as f:  # vuln-code-snippet target-line ChainScenario0128A
        content = f.read()
# vuln-code-snippet end ChainScenario0128A
    return jsonify({"name": log_name, "content": content})
