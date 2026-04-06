"""Archive extraction blueprint -- SAFE variant.

POST /extract validates each ZIP member path to ensure it resolves within
the target directory before extracting.  GET /extract/<name> canonicalizes
the path and verifies it stays within the extraction base.

Chain: broken -- member paths validated before extraction, read path canonicalized
CWE-22: Improper Limitation of a Pathname to a Restricted Directory (remediated)
"""
import os
import zipfile
from flask import Blueprint, request, jsonify
import config

extractor_bp = Blueprint("extractor", __name__)


@extractor_bp.route("/extract", methods=["POST"])
def extract_archive():
    """Accept a ZIP file and extract it.

    FIXED: each member path is resolved against the target directory and
    rejected if it would land outside that directory.
    """
    uploaded = request.files.get("archive")
    if not uploaded:
        return jsonify({"error": "No archive provided"}), 400
    archive_path = os.path.join(config.UPLOAD_DIR, uploaded.filename)
    uploaded.save(archive_path)
    target_dir = os.path.realpath(
        os.path.join(config.EXTRACT_BASE_DIR, uploaded.filename.rsplit(".", 1)[0])
    )
    os.makedirs(target_dir, exist_ok=True)
    with zipfile.ZipFile(archive_path) as zf:
        for member in zf.infolist():
            member_path = os.path.realpath(os.path.join(target_dir, member.filename))
            if not member_path.startswith(target_dir + os.sep):
                return jsonify({"error": "Invalid archive entry"}), 400
            zf.extract(member, target_dir)
    return jsonify({"extracted_to": target_dir})


@extractor_bp.route("/extract/<path:filename>")
def read_extracted(filename):
    """Read back a previously extracted file.

    FIXED: path is canonicalized and constrained to the extraction base.
    """
    base = os.path.realpath(config.EXTRACT_BASE_DIR)
    file_path = os.path.realpath(os.path.join(config.EXTRACT_BASE_DIR, filename))
    if not file_path.startswith(base + os.sep):
        return jsonify({"error": "Access denied"}), 403
# vuln-code-snippet start chain_zip_traversal_safe
    with open(file_path, "r") as f:  # vuln-code-snippet safe-line chain_zip_traversal_safe
        content = f.read()
# vuln-code-snippet end chain_zip_traversal_safe
    return jsonify({"filename": filename, "content": content})
