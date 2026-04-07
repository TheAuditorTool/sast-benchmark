"""Flask views for markdown_ssti scenario -- SAFE variant.

POST /markdown passes the client-supplied source directly to
renderer.render_markdown, which performs no Jinja2 preprocessing.
Jinja2 expressions in the source are treated as literal Markdown text.

Chain broken: no Jinja2 preprocessing -> no SSTI
"""
from flask import Blueprint, request, jsonify
from renderer import render_markdown

markdown_bp = Blueprint("markdown", __name__)


# vuln-code-snippet start chain_markdown_ssti_safe
@markdown_bp.route("/markdown", methods=["POST"])
def markdown():
    """Render user-supplied Markdown directly to HTML without template preprocessing."""
    source = request.json.get("source", "")
    html = render_markdown(source)  # vuln-code-snippet safe-line chain_markdown_ssti_safe
    return jsonify({"html": html})
# vuln-code-snippet end chain_markdown_ssti_safe
