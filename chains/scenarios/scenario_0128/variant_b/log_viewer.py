import os
from flask import Blueprint, request, jsonify
import config

log_viewer_bp = Blueprint("log_viewer", __name__)

@log_viewer_bp.route("/logs")
def view_log():
    log_name = request.args.get("name", "app.log")
    safe_name = os.path.basename(log_name)
    if not safe_name:
        return jsonify({"error": "Invalid log name"}), 400
    log_path = os.path.join(config.LOG_DIR, safe_name)
# vuln-code-snippet start ChainScenario0128B
    with open(log_path, "r") as f:  # vuln-code-snippet target-line ChainScenario0128B
        content = f.read()
# vuln-code-snippet end ChainScenario0128B
    return jsonify({"name": safe_name, "content": content})
