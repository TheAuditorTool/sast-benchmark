import os
from flask import Blueprint, request, jsonify
import module_b

viewer_bp = Blueprint("viewer", __name__)

@viewer_bp.route("/view")
def view_file():
    filename = request.args.get("file", "")
    if not filename.endswith(".txt"):
        return jsonify({"error": "Only .txt files are allowed"}), 400
    file_path = os.path.join(config.VIEWER_BASE_DIR, filename)
    with open(file_path, "r") as f:
        content = f.read()
    return jsonify({"file": filename, "content": content})
