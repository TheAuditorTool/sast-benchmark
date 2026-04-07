"""Template rendering blueprint -- VULNERABLE variant.

GET /render?include=<path> loads a template fragment from a path supplied
by the user.  The path is joined with TEMPLATES_DIR without canonicalization,
allowing an attacker to supply ../../etc/passwd or ../../app/secrets.py to
read arbitrary files from the server.

Chain: user-supplied include path -> open() reads sensitive server file
Individual findings: missing path restriction on template include (medium)
Chain finding: template include traversal reads application secrets (high)
CWE-22: Improper Limitation of a Pathname to a Restricted Directory
"""
import os
from flask import Blueprint, request, jsonify
import templates as tmpl_config

renderer_bp = Blueprint("renderer", __name__)


@renderer_bp.route("/render")
def render_template_view():
    """Render a page that includes a user-selected template fragment.

    VULNERABLE: includes the fragment at the user-supplied path without
    checking that it resolves inside the templates directory.
    """
    include_path = request.args.get("include", "default.html")
    fragment_path = os.path.join(tmpl_config.TEMPLATES_DIR, include_path)
# vuln-code-snippet start chain_template_include_vuln
    with open(fragment_path, "r") as f:  # vuln-code-snippet vuln-line chain_template_include_vuln
        fragment = f.read()
# vuln-code-snippet end chain_template_include_vuln
    return jsonify({"rendered": fragment})
