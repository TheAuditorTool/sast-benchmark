import os
from flask import Blueprint, request, jsonify
import module_b

log_viewer_bp = Blueprint("log_viewer", __name__)

@log_viewer_bp.route("/logs")
def view_log():
    log_name = request.args.get("name", "app.log")
    log_path = os.path.join(config.LOG_DIR, log_name)
    with open(log_path, "r") as f:
        content = f.read()
    return jsonify({"name": log_name, "content": content})
