import os
from flask import Blueprint, jsonify
import module_b

static_handler_bp = Blueprint("static_handler", __name__)

@static_handler_bp.route("/asset/<path:filepath>")
def serve_asset(filepath):
    file_path = os.path.join(config.PROJECT_ROOT, filepath)
    if not os.path.isfile(file_path):
        return jsonify({"error": "Not found"}), 404
    with open(file_path, "r") as f:
        content = f.read()
    return jsonify({"path": filepath, "content": content})
