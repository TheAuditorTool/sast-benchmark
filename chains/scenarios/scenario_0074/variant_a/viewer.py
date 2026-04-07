import os
from flask import Blueprint, request, jsonify
import config

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
# vuln-code-snippet start ChainScenario0074A
    with open(file_path, "r") as f:  # vuln-code-snippet target-line ChainScenario0074A
        content = f.read()
# vuln-code-snippet end ChainScenario0074A
    return jsonify({"file": filename, "content": content})
