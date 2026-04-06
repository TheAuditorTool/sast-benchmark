"""File download blueprint for the download service -- VULNERABLE variant.

GET /downloads/<filename> resolves the filename relative to DOWNLOAD_BASE_DIR
without canonicalizing the path.  An attacker can supply a filename like
../../etc/passwd to escape the base directory and read arbitrary files.

Chain: user-supplied filename -> os.path.join -> open() reads sensitive file
Individual findings: missing path canonicalization (medium)
Chain finding: path traversal enables reading /etc/passwd or app secrets (high)
CWE-22: Improper Limitation of a Pathname to a Restricted Directory
"""
import os
from flask import Blueprint, request, jsonify, send_file
import config

downloads_bp = Blueprint("downloads", __name__)


@downloads_bp.route("/downloads/<path:filename>")
def download_file(filename):
    """Serve a file from the downloads directory.

    VULNERABLE: joins user-supplied filename with base dir without
    canonicalizing, so ../../../etc/passwd traverses out of the base dir.
    """
    file_path = os.path.join(config.DOWNLOAD_BASE_DIR, filename)
# vuln-code-snippet start chain_dotdot_config_vuln
    with open(file_path, "r") as f:  # vuln-code-snippet vuln-line chain_dotdot_config_vuln
        content = f.read()
# vuln-code-snippet end chain_dotdot_config_vuln
    return jsonify({"filename": filename, "content": content})
