import os
from flask import Blueprint, request, jsonify
import module_b

viewer_bp = Blueprint("viewer", __name__)

@viewer_bp.route("/view")
def view_file():
    filename = request.args.get("file", "")
    if "\x00" in filename:
        return jsonify({"error": "Invalid filename"}), 400
    if not filename.endswith(".txt"):
        return jsonify({"error": "Only .txt files are allowed"}), 400
    base = os.path.realpath(config.VIEWER_BASE_DIR)
    file_path = os.path.realpath(os.path.join(config.VIEWER_BASE_DIR, filename))
    if not file_path.startswith(base + os.sep):
        return jsonify({"error": "Access denied"}), 403
    with open(file_path, "r") as f:
        content = f.read()
    return jsonify({"file": filename, "content": content})
