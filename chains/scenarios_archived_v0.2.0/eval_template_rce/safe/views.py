"""Flask views for eval_template_rce scenario -- SAFE variant.

POST /render passes the client-supplied template to engine.render_template,
which performs only dict key lookups. Expressions are never evaluated,
so injected OS calls cannot execute.

Chain broken: engine uses dict lookup -> no eval -> no RCE
"""
from flask import Blueprint, request, jsonify
from engine import render_template

render_bp = Blueprint("render", __name__)


# vuln-code-snippet start chain_eval_template_safe
@render_bp.route("/render", methods=["POST"])
def render():
    """Render a template string using safe context key substitution."""
    template = request.json.get("template", "")
    context = request.json.get("context", {})
    output = render_template(template, context)  # vuln-code-snippet safe-line chain_eval_template_safe
    return jsonify({"output": output})
# vuln-code-snippet end chain_eval_template_safe
