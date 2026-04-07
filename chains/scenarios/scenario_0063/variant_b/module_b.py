import os
from flask import Blueprint, jsonify
import module_c as app_secrets

file_server_bp = Blueprint("file_server", __name__)

@file_server_bp.route("/files/<path:filepath>")
def serve_file(filepath):
    if ".." in filepath:
        return jsonify({"error": "Invalid path"}), 400
    full_path = os.path.join(app_secrets.FILES_BASE_DIR, filepath)
    with open(full_path, "r") as f:
        content = f.read()
    return jsonify({"path": filepath, "content": content})
