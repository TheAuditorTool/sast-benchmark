import os
from flask import Blueprint, request, jsonify, send_file
import config

downloads_bp = Blueprint("downloads", __name__)

@downloads_bp.route("/downloads/<path:filename>")
def download_file(filename):
    file_path = os.path.join(config.DOWNLOAD_BASE_DIR, filename)
# vuln-code-snippet start ChainScenario0189A
    with open(file_path, "r") as f:  # vuln-code-snippet target-line ChainScenario0189A
        content = f.read()
# vuln-code-snippet end ChainScenario0189A
    return jsonify({"filename": filename, "content": content})
