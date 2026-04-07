import os
import zipfile
from flask import Blueprint, request, jsonify
import config

extractor_bp = Blueprint("extractor", __name__)

@extractor_bp.route("/extract", methods=["POST"])
def extract_archive():
    uploaded = request.files.get("archive")
    if not uploaded:
        return jsonify({"error": "No archive provided"}), 400
    archive_path = os.path.join(config.UPLOAD_DIR, uploaded.filename)
    uploaded.save(archive_path)
    target_dir = os.path.join(config.EXTRACT_BASE_DIR, uploaded.filename.rsplit(".", 1)[0])
    os.makedirs(target_dir, exist_ok=True)
    with zipfile.ZipFile(archive_path) as zf:
        zf.extractall(target_dir)
    return jsonify({"extracted_to": target_dir})

@extractor_bp.route("/extract/<path:filename>")
def read_extracted(filename):
    file_path = os.path.join(config.EXTRACT_BASE_DIR, filename)
# vuln-code-snippet start ChainScenario0171A
    with open(file_path, "r") as f:  # vuln-code-snippet target-line ChainScenario0171A
        content = f.read()
# vuln-code-snippet end ChainScenario0171A
    return jsonify({"filename": filename, "content": content})
