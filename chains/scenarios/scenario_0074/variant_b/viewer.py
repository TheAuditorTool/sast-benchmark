import os
from flask import Blueprint, request, jsonify
import config

viewer_bp = Blueprint("viewer", __name__)

@viewer_bp.route("/view")
def view_file():
    filename = request.args.get("file", "")
    if not filename.endswith(".txt"):
        return jsonify({"error": "Only .txt files are allowed"}), 400
    file_path = os.path.join(config.VIEWER_BASE_DIR, filename)
# vuln-code-snippet start ChainScenario0074B
    with open(file_path, "r") as f:  # vuln-code-snippet target-line ChainScenario0074B
        content = f.read()
# vuln-code-snippet end ChainScenario0074B
    return jsonify({"file": filename, "content": content})
