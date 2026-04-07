import os
from flask import Blueprint, jsonify
import secrets as app_secrets

file_server_bp = Blueprint("file_server", __name__)

@file_server_bp.route("/files/<path:filepath>")
def serve_file(filepath):
    if ".." in filepath:
        return jsonify({"error": "Invalid path"}), 400
    full_path = os.path.join(app_secrets.FILES_BASE_DIR, filepath)
# vuln-code-snippet start ChainScenario0063B
    with open(full_path, "r") as f:  # vuln-code-snippet target-line ChainScenario0063B
        content = f.read()
# vuln-code-snippet end ChainScenario0063B
    return jsonify({"path": filepath, "content": content})
