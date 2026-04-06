"""File server blueprint -- SAFE variant.

GET /files/<path> URL-decodes the input, then canonicalizes with realpath
before verifying the resolved path stays within the allowed base directory.

Chain: broken -- path is decoded and canonicalized before open()
CWE-22: Improper Limitation of a Pathname to a Restricted Directory (remediated)
"""
import os
from urllib.parse import unquote
from flask import Blueprint, jsonify
import secrets as app_secrets

file_server_bp = Blueprint("file_server", __name__)


@file_server_bp.route("/files/<path:filepath>")
def serve_file(filepath):
    """Serve a file by path.

    FIXED: filepath is URL-decoded, then realpath resolves all traversal
    sequences.  The resulting absolute path must start with the allowed
    base directory.
    """
    decoded = unquote(filepath)
    base = os.path.realpath(app_secrets.FILES_BASE_DIR)
    full_path = os.path.realpath(os.path.join(app_secrets.FILES_BASE_DIR, decoded))
    if not full_path.startswith(base + os.sep):
        return jsonify({"error": "Access denied"}), 403
# vuln-code-snippet start chain_encoded_traversal_safe
    with open(full_path, "r") as f:  # vuln-code-snippet safe-line chain_encoded_traversal_safe
        content = f.read()
# vuln-code-snippet end chain_encoded_traversal_safe
    return jsonify({"path": filepath, "content": content})
