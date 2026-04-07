"""Flask views for string_template_rce scenario -- VULNERABLE variant.

POST /template accepts a JSON body with a 'template' field. The template
is processed with string.Template.substitute using globals() as the
substitution mapping, allowing an attacker to exfiltrate globals.

Chain: POST /template -> engine.render_template(template) -> substitute(globals()) -> disclosure
"""
from flask import Blueprint, request, jsonify
from engine import render_template

template_bp = Blueprint("template", __name__)


# vuln-code-snippet start chain_string_template_vuln
@template_bp.route("/template", methods=["POST"])
def apply_template():
    """Render a string.Template with the module globals as substitution mapping."""
    template_str = request.json.get("template", "$APP_NAME")
    result = render_template(template_str)  # vuln-code-snippet vuln-line chain_string_template_vuln
    return jsonify({"result": result})
# vuln-code-snippet end chain_string_template_vuln
