import os
from urllib.parse import unquote
from flask import Blueprint, jsonify
import module_c as app_secrets

file_server_bp = Blueprint("file_server", __name__)

@file_server_bp.route("/files/<path:filepath>")
def serve_file(filepath):
    decoded = unquote(filepath)
    base = os.path.realpath(app_secrets.FILES_BASE_DIR)
    full_path = os.path.realpath(os.path.join(app_secrets.FILES_BASE_DIR, decoded))
    if not full_path.startswith(base + os.sep):
        return jsonify({"error": "Access denied"}), 403
    with open(full_path, "r") as f:
        content = f.read()
    return jsonify({"path": filepath, "content": content})
