"""Static file serving blueprint -- VULNERABLE variant.

GET /static/<filename> opens the file directly without checking whether
the path is a symbolic link.  An attacker who can place a symlink in the
upload directory (e.g. pointing to /etc/shadow or application secrets)
can read arbitrary files through the static serving endpoint.

Chain: symlink in upload dir -> open() follows symlink -> reads sensitive file
Individual findings: missing symlink check before file open (medium)
Chain finding: symlink traversal reads /etc/shadow or app credential files (high)
CWE-22: Improper Limitation of a Pathname to a Restricted Directory
"""
import os
from flask import Blueprint, jsonify
import storage

serve_bp = Blueprint("serve", __name__)


@serve_bp.route("/static/<path:filename>")
def serve_static(filename):
    """Serve a static file.

    VULNERABLE: opens the resolved path without checking if it is a symlink,
    so a symlink planted in the uploads directory can redirect reads to
    any file readable by the server process.
    """
    file_path = os.path.join(storage.STATIC_ROOT, filename)
    if not os.path.exists(file_path):
        return jsonify({"error": "File not found"}), 404
# vuln-code-snippet start chain_symlink_follow_vuln
    with open(file_path, "r") as f:  # vuln-code-snippet vuln-line chain_symlink_follow_vuln
        content = f.read()
# vuln-code-snippet end chain_symlink_follow_vuln
    return jsonify({"filename": filename, "content": content})
