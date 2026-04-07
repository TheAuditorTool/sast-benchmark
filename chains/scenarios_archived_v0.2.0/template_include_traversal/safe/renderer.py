"""Template rendering blueprint -- SAFE variant.

GET /render?include=<path> canonicalizes the requested fragment path and
verifies it resolves within the configured templates directory before opening.

Chain: broken -- path canonicalized and restricted to templates dir before open()
CWE-22: Improper Limitation of a Pathname to a Restricted Directory (remediated)
"""
import os
from flask import Blueprint, request, jsonify
import templates as tmpl_config

renderer_bp = Blueprint("renderer", __name__)


@renderer_bp.route("/render")
def render_template_view():
    """Render a page that includes a user-selected template fragment.

    FIXED: realpath resolves traversal sequences; the result must start
    with the configured templates directory.
    """
    include_path = request.args.get("include", "default.html")
    base = os.path.realpath(tmpl_config.TEMPLATES_DIR)
    fragment_path = os.path.realpath(os.path.join(tmpl_config.TEMPLATES_DIR, include_path))
    if not fragment_path.startswith(base + os.sep):
        return jsonify({"error": "Template not found"}), 404
# vuln-code-snippet start chain_template_include_safe
    with open(fragment_path, "r") as f:  # vuln-code-snippet safe-line chain_template_include_safe
        fragment = f.read()
# vuln-code-snippet end chain_template_include_safe
    return jsonify({"rendered": fragment})
