"""Static file handler blueprint -- SAFE variant.

GET /asset/<path> constrains file serving to the public/ subdirectory.
The resolved path must start with PUBLIC_DIR after canonicalization.

Chain: broken -- path restricted to PUBLIC_DIR before open()
CWE-22: Improper Limitation of a Pathname to a Restricted Directory (remediated)
"""
import os
from flask import Blueprint, jsonify
import config

static_handler_bp = Blueprint("static_handler", __name__)


@static_handler_bp.route("/asset/<path:filepath>")
def serve_asset(filepath):
    """Serve a static asset.

    FIXED: filepath is resolved within PUBLIC_DIR, not PROJECT_ROOT.
    realpath + prefix check prevents traversal outside the public directory.
    """
    base = os.path.realpath(config.PUBLIC_DIR)
    file_path = os.path.realpath(os.path.join(config.PUBLIC_DIR, filepath))
    if not file_path.startswith(base + os.sep):
        return jsonify({"error": "Access denied"}), 403
    if not os.path.isfile(file_path):
        return jsonify({"error": "Not found"}), 404
# vuln-code-snippet start chain_static_bypass_safe
    with open(file_path, "r") as f:  # vuln-code-snippet safe-line chain_static_bypass_safe
        content = f.read()
# vuln-code-snippet end chain_static_bypass_safe
    return jsonify({"path": filepath, "content": content})
