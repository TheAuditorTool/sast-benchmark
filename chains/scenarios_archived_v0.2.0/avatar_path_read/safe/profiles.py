"""User profile blueprint -- SAFE variant.

GET /avatar?path=<path> canonicalizes the path and verifies it resolves
within the configured avatars directory before reading.

Chain: broken -- path canonicalized and restricted to AVATARS_DIR before open()
CWE-22: Improper Limitation of a Pathname to a Restricted Directory (remediated)
"""
import os
from flask import Blueprint, request, jsonify, Response
import file_utils

profiles_bp = Blueprint("profiles", __name__)


@profiles_bp.route("/avatar")
def get_avatar():
    """Return avatar image bytes for the given path.

    FIXED: realpath resolves traversal sequences; the result must start
    with the avatars directory before the file is opened.
    """
    avatar_path = request.args.get("path", "")
    if not avatar_path:
        return jsonify({"error": "path required"}), 400
    base = os.path.realpath(file_utils.AVATARS_DIR)
    full_path = os.path.realpath(os.path.join(file_utils.AVATARS_DIR, avatar_path))
    if not full_path.startswith(base + os.sep):
        return jsonify({"error": "Access denied"}), 403
# vuln-code-snippet start chain_avatar_path_safe
    with open(full_path, "rb") as f:  # vuln-code-snippet safe-line chain_avatar_path_safe
        data = f.read()
# vuln-code-snippet end chain_avatar_path_safe
    return Response(data, mimetype="image/png")
