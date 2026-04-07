import os
from flask import Blueprint, jsonify
import module_b

static_handler_bp = Blueprint("static_handler", __name__)

@static_handler_bp.route("/asset/<path:filepath>")
def serve_asset(filepath):
    base = os.path.realpath(config.PUBLIC_DIR)
    file_path = os.path.realpath(os.path.join(config.PUBLIC_DIR, filepath))
    if not file_path.startswith(base + os.sep):
        return jsonify({"error": "Access denied"}), 403
    if not os.path.isfile(file_path):
        return jsonify({"error": "Not found"}), 404
    with open(file_path, "r") as f:
        content = f.read()
    return jsonify({"path": filepath, "content": content})
