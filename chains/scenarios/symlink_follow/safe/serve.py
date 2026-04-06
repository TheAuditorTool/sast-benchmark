"""Static file serving blueprint -- SAFE variant.

GET /static/<filename> checks that the resolved path is not a symbolic link
and that it stays within the allowed static root before opening.

Chain: broken -- symlinks rejected and path constrained before open()
CWE-22: Improper Limitation of a Pathname to a Restricted Directory (remediated)
"""
import os
from flask import Blueprint, jsonify
import storage

serve_bp = Blueprint("serve", __name__)


@serve_bp.route("/static/<path:filename>")
def serve_static(filename):
    """Serve a static file.

    FIXED: checks os.path.islink() and rejects symlinks.  Also verifies
    the canonical path stays within the configured static root.
    """
    file_path = os.path.realpath(os.path.join(storage.STATIC_ROOT, filename))
    base = os.path.realpath(storage.STATIC_ROOT)
    if not file_path.startswith(base + os.sep):
        return jsonify({"error": "Access denied"}), 403
    raw_path = os.path.join(storage.STATIC_ROOT, filename)
    if os.path.islink(raw_path):
        return jsonify({"error": "Symbolic links are not permitted"}), 403
    if not os.path.exists(file_path):
        return jsonify({"error": "File not found"}), 404
# vuln-code-snippet start chain_symlink_follow_safe
    with open(file_path, "r") as f:  # vuln-code-snippet safe-line chain_symlink_follow_safe
        content = f.read()
# vuln-code-snippet end chain_symlink_follow_safe
    return jsonify({"filename": filename, "content": content})
