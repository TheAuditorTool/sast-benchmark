"""Flask views for mako_ssti_rce scenario -- VULNERABLE variant.

POST /render accepts a JSON body with a 'template' field containing
a Mako template string that is compiled and rendered server-side.
An attacker can inject <% import os; os.system('id') %> to achieve RCE.

Chain: POST /render -> renderer.render_page(template) -> RCE
"""
from flask import Blueprint, request, jsonify
from renderer import render_page

render_bp = Blueprint("render", __name__)


# vuln-code-snippet start chain_mako_ssti_vuln
@render_bp.route("/render", methods=["POST"])
def render():
    """Render a Mako template string supplied by the client."""
    template_source = request.json.get("template", "")
    output = render_page(template_source)  # vuln-code-snippet vuln-line chain_mako_ssti_vuln
    return jsonify({"output": output})
# vuln-code-snippet end chain_mako_ssti_vuln
