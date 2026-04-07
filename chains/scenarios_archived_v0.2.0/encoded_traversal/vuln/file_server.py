"""File server blueprint -- VULNERABLE variant.

GET /files/<path> checks the raw path for ".." sequences but does not
URL-decode the input first.  A request to /files/..%2F..%2Fetc%2Fpasswd
passes the string check while still resolving to /etc/passwd after the OS
processes the path separator encoded as %2F.

Chain: URL-encoded traversal bypass -> open() reads sensitive file
Individual findings: incomplete input validation (medium)
Chain finding: encoded traversal reads server credentials or /etc/passwd (high)
CWE-22: Improper Limitation of a Pathname to a Restricted Directory
"""
import os
from flask import Blueprint, jsonify
import secrets as app_secrets

file_server_bp = Blueprint("file_server", __name__)


@file_server_bp.route("/files/<path:filepath>")
def serve_file(filepath):
    """Serve a file by path.

    VULNERABLE: checks raw 'filepath' for '..' but Flask's routing already
    percent-decoded the value, so the check is applied to the decoded string.
    Sending ..%2F in the raw URL still results in ../ after decoding and
    the check fails to catch it because the percent-encoded form was never seen.
    """
    if ".." in filepath:
        return jsonify({"error": "Invalid path"}), 400
    full_path = os.path.join(app_secrets.FILES_BASE_DIR, filepath)
# vuln-code-snippet start chain_encoded_traversal_vuln
    with open(full_path, "r") as f:  # vuln-code-snippet vuln-line chain_encoded_traversal_vuln
        content = f.read()
# vuln-code-snippet end chain_encoded_traversal_vuln
    return jsonify({"path": filepath, "content": content})
