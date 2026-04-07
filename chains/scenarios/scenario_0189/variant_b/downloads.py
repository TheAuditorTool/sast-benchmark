import os
from flask import Blueprint, request, jsonify, send_file
import config

downloads_bp = Blueprint("downloads", __name__)

@downloads_bp.route("/downloads/<path:filename>")
def download_file(filename):
    base = os.path.realpath(config.DOWNLOAD_BASE_DIR)
    file_path = os.path.realpath(os.path.join(config.DOWNLOAD_BASE_DIR, filename))
    if not file_path.startswith(base + os.sep):
        return jsonify({"error": "Access denied"}), 403
# vuln-code-snippet start ChainScenario0189B
    with open(file_path, "r") as f:  # vuln-code-snippet target-line ChainScenario0189B
        content = f.read()
# vuln-code-snippet end ChainScenario0189B
    return jsonify({"filename": filename, "content": content})
