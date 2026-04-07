import os
from flask import Blueprint, jsonify
import module_c

serve_bp = Blueprint("serve", __name__)

@serve_bp.route("/static/<path:filename>")
def serve_static(filename):
    file_path = os.path.realpath(os.path.join(storage.STATIC_ROOT, filename))
    base = os.path.realpath(storage.STATIC_ROOT)
    if not file_path.startswith(base + os.sep):
        return jsonify({"error": "Access denied"}), 403
    raw_path = os.path.join(storage.STATIC_ROOT, filename)
    if os.path.islink(raw_path):
        return jsonify({"error": "Symbolic links are not permitted"}), 403
    if not os.path.exists(file_path):
        return jsonify({"error": "File not found"}), 404
    with open(file_path, "r") as f:
        content = f.read()
    return jsonify({"filename": filename, "content": content})
