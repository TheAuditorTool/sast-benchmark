import os
from flask import Blueprint, jsonify
import module_c

serve_bp = Blueprint("serve", __name__)

@serve_bp.route("/static/<path:filename>")
def serve_static(filename):
    file_path = os.path.join(storage.STATIC_ROOT, filename)
    if not os.path.exists(file_path):
        return jsonify({"error": "File not found"}), 404
    with open(file_path, "r") as f:
        content = f.read()
    return jsonify({"filename": filename, "content": content})
