import os
from urllib.parse import unquote
from flask import Blueprint, jsonify
import secrets as app_secrets

file_server_bp = Blueprint("file_server", __name__)

@file_server_bp.route("/files/<path:filepath>")
def serve_file(filepath):
    decoded = unquote(filepath)
    base = os.path.realpath(app_secrets.FILES_BASE_DIR)
    full_path = os.path.realpath(os.path.join(app_secrets.FILES_BASE_DIR, decoded))
    if not full_path.startswith(base + os.sep):
        return jsonify({"error": "Access denied"}), 403
# vuln-code-snippet start ChainScenario0063A
    with open(full_path, "r") as f:  # vuln-code-snippet target-line ChainScenario0063A
        content = f.read()
# vuln-code-snippet end ChainScenario0063A
    return jsonify({"path": filepath, "content": content})
