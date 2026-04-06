"""Flask views for string_template_rce scenario -- SAFE variant.

POST /template passes the client-supplied template to engine.render_template,
which calls safe_substitute with a narrow context. Globals are not
accessible; unknown placeholders are left unchanged.

Chain broken: safe_substitute with restricted context -> no globals access -> no disclosure
"""
from flask import Blueprint, request, jsonify
from engine import render_template

template_bp = Blueprint("template", __name__)


# vuln-code-snippet start chain_string_template_safe
@template_bp.route("/template", methods=["POST"])
def apply_template():
    """Render a string.Template with a safe restricted substitution context."""
    template_str = request.json.get("template", "$APP_NAME")
    result = render_template(template_str)  # vuln-code-snippet safe-line chain_string_template_safe
    return jsonify({"result": result})
# vuln-code-snippet end chain_string_template_safe
