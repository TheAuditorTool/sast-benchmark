"""Flask views for markdown_ssti scenario -- VULNERABLE variant.

POST /markdown accepts a JSON body with a 'source' field containing
Markdown text. Before HTML conversion the source is rendered as a
Jinja2 template, allowing SSTI-based RCE via injected expressions.

Chain: POST /markdown -> renderer.render_markdown(source) -> from_string -> RCE
"""
from flask import Blueprint, request, jsonify
from renderer import render_markdown

markdown_bp = Blueprint("markdown", __name__)


# vuln-code-snippet start chain_markdown_ssti_vuln
@markdown_bp.route("/markdown", methods=["POST"])
def markdown():
    """Render user-supplied Markdown after Jinja2 template preprocessing."""
    source = request.json.get("source", "")
    html = render_markdown(source)  # vuln-code-snippet vuln-line chain_markdown_ssti_vuln
    return jsonify({"html": html})
# vuln-code-snippet end chain_markdown_ssti_vuln
