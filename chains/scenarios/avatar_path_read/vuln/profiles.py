"""User profile blueprint -- VULNERABLE variant.

GET /avatar?path=<path> reads the avatar image at the user-supplied path.
No validation is applied, so an attacker can supply an arbitrary path such
as ../../etc/passwd or ../../app/secrets.py to read any server file.

Chain: user-supplied avatar path -> open() reads arbitrary file
Individual findings: user-controlled path passed directly to file read (medium)
Chain finding: avatar path traversal reads application secrets or system files (high)
CWE-22: Improper Limitation of a Pathname to a Restricted Directory
"""
import os
from flask import Blueprint, request, jsonify, Response
import file_utils

profiles_bp = Blueprint("profiles", __name__)


@profiles_bp.route("/avatar")
def get_avatar():
    """Return avatar image bytes for the given path.

    VULNERABLE: passes the user-supplied path directly to read_binary_file
    without canonicalizing or restricting to the avatars directory.
    """
    avatar_path = request.args.get("path", "")
    if not avatar_path:
        return jsonify({"error": "path required"}), 400
    full_path = os.path.join(file_utils.AVATARS_DIR, avatar_path)
# vuln-code-snippet start chain_avatar_path_vuln
    with open(full_path, "rb") as f:  # vuln-code-snippet vuln-line chain_avatar_path_vuln
        data = f.read()
# vuln-code-snippet end chain_avatar_path_vuln
    return Response(data, mimetype="image/png")
