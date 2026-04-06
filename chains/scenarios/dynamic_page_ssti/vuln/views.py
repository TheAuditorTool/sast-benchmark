"""Flask views for dynamic_page_ssti scenario -- VULNERABLE variant.

POST /page accepts a JSON body with 'content' and 'context' fields.
The content is rendered directly as a Jinja2 template, so injected
expressions can traverse globals and execute OS commands.

Chain: POST /page -> renderer.render_page(content) -> from_string -> RCE
"""
from flask import Blueprint, request, jsonify
from renderer import render_page

page_bp = Blueprint("page", __name__)


# vuln-code-snippet start chain_dynamic_page_vuln
@page_bp.route("/page", methods=["POST"])
def page():
    """Render a CMS page using its stored content as a Jinja2 template."""
    content = request.json.get("content", "")
    context = request.json.get("context", {})
    output = render_page(content, context)  # vuln-code-snippet vuln-line chain_dynamic_page_vuln
    return jsonify({"html": output})
# vuln-code-snippet end chain_dynamic_page_vuln
