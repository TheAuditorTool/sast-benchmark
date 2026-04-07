import os
from flask import Blueprint, jsonify
import storage

serve_bp = Blueprint("serve", __name__)

@serve_bp.route("/static/<path:filename>")
def serve_static(filename):
    file_path = os.path.join(storage.STATIC_ROOT, filename)
    if not os.path.exists(file_path):
        return jsonify({"error": "File not found"}), 404
# vuln-code-snippet start ChainScenario0044A
    with open(file_path, "r") as f:  # vuln-code-snippet target-line ChainScenario0044A
        content = f.read()
# vuln-code-snippet end ChainScenario0044A
    return jsonify({"filename": filename, "content": content})
