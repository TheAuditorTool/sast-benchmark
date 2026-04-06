"""Archive extraction blueprint -- VULNERABLE variant.

POST /extract accepts a ZIP archive upload and extracts all members to a
target directory without inspecting member names.  A crafted archive whose
entries contain paths like ../../etc/cron.d/backdoor can write files
outside the intended extraction directory.  The subsequent GET /extract/<name>
endpoint reads back extracted files and can be used to read any file that
was written outside the extraction root.

Chain: malicious ZIP member path -> extractall() writes outside dir -> open() reads secret
Individual findings: missing member path validation (medium)
Chain finding: zip slip writes + reads app secrets or config files (high)
CWE-22: Improper Limitation of a Pathname to a Restricted Directory
"""
import os
import zipfile
from flask import Blueprint, request, jsonify
import config

extractor_bp = Blueprint("extractor", __name__)


@extractor_bp.route("/extract", methods=["POST"])
def extract_archive():
    """Accept a ZIP file and extract it.

    VULNERABLE: calls extractall() without validating member paths, enabling
    zip-slip writes to arbitrary locations.
    """
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
    """Read back a previously extracted file.

    VULNERABLE: joins filename directly to extract base without canonicalizing.
    Combined with zip-slip, can read files outside the extraction directory.
    """
    file_path = os.path.join(config.EXTRACT_BASE_DIR, filename)
# vuln-code-snippet start chain_zip_traversal_vuln
    with open(file_path, "r") as f:  # vuln-code-snippet vuln-line chain_zip_traversal_vuln
        content = f.read()
# vuln-code-snippet end chain_zip_traversal_vuln
    return jsonify({"filename": filename, "content": content})
