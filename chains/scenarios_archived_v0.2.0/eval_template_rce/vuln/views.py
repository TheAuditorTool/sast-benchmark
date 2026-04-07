"""Flask views for eval_template_rce scenario -- VULNERABLE variant.

POST /render accepts a JSON body with a 'template' field. The custom
engine evaluates every {{ expr }} block with eval(), so an attacker
can achieve RCE by injecting OS calls into the template expression.

Chain: POST /render -> engine.render_template(template) -> eval -> RCE
"""
from flask import Blueprint, request, jsonify
from engine import render_template

render_bp = Blueprint("render", __name__)


# vuln-code-snippet start chain_eval_template_vuln
@render_bp.route("/render", methods=["POST"])
def render():
    """Render a custom template string using the eval-based engine."""
    template = request.json.get("template", "")
    context = request.json.get("context", {})
    output = render_template(template, context)  # vuln-code-snippet vuln-line chain_eval_template_vuln
    return jsonify({"output": output})
# vuln-code-snippet end chain_eval_template_vuln
