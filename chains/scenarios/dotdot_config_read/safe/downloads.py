"""File download blueprint for the download service -- SAFE variant.

GET /downloads/<filename> resolves the filename relative to DOWNLOAD_BASE_DIR
and then verifies the canonical path starts with the allowed base directory.
Path traversal sequences are neutralized by os.path.realpath.

Chain: broken -- canonicalized path validated before open()
CWE-22: Improper Limitation of a Pathname to a Restricted Directory (remediated)
"""
import os
from flask import Blueprint, request, jsonify, send_file
import config

downloads_bp = Blueprint("downloads", __name__)


@downloads_bp.route("/downloads/<path:filename>")
def download_file(filename):
    """Serve a file from the downloads directory.

    FIXED: realpath resolves all .. sequences and symlinks; the resulting
    path is checked to start with the allowed base before opening.
    """
    base = os.path.realpath(config.DOWNLOAD_BASE_DIR)
    file_path = os.path.realpath(os.path.join(config.DOWNLOAD_BASE_DIR, filename))
    if not file_path.startswith(base + os.sep):
        return jsonify({"error": "Access denied"}), 403
# vuln-code-snippet start chain_dotdot_config_safe
    with open(file_path, "r") as f:  # vuln-code-snippet safe-line chain_dotdot_config_safe
        content = f.read()
# vuln-code-snippet end chain_dotdot_config_safe
    return jsonify({"filename": filename, "content": content})
