"""Static file handler blueprint -- VULNERABLE variant.

GET /asset/<path> serves files relative to the project root rather than
the dedicated public/ subdirectory.  An attacker requesting
/asset/config.py or /asset/../etc/passwd can read application source
files and system files.

Chain: user-supplied path -> open() relative to project root -> serves app source/secrets
Individual findings: overly broad static file root (medium)
Chain finding: static handler serves application source code and credentials (high)
CWE-22: Improper Limitation of a Pathname to a Restricted Directory
"""
import os
from flask import Blueprint, jsonify
import config

static_handler_bp = Blueprint("static_handler", __name__)


@static_handler_bp.route("/asset/<path:filepath>")
def serve_asset(filepath):
    """Serve a static asset.

    VULNERABLE: resolves filepath relative to PROJECT_ROOT rather than
    PUBLIC_DIR, so requests for config.py or other source files succeed.
    """
    file_path = os.path.join(config.PROJECT_ROOT, filepath)
    if not os.path.isfile(file_path):
        return jsonify({"error": "Not found"}), 404
# vuln-code-snippet start chain_static_bypass_vuln
    with open(file_path, "r") as f:  # vuln-code-snippet vuln-line chain_static_bypass_vuln
        content = f.read()
# vuln-code-snippet end chain_static_bypass_vuln
    return jsonify({"path": filepath, "content": content})
