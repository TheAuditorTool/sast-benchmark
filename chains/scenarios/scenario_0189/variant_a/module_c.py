import os
from flask import Blueprint, request, jsonify, send_file
import module_b

downloads_bp = Blueprint("downloads", __name__)

@downloads_bp.route("/downloads/<path:filename>")
def download_file(filename):
    file_path = os.path.join(config.DOWNLOAD_BASE_DIR, filename)
    with open(file_path, "r") as f:
        content = f.read()
    return jsonify({"filename": filename, "content": content})
