"""Flask views for dynamic_page_ssti scenario -- SAFE variant.

POST /page passes the client-supplied content as a context variable to
renderer.render_page. The static template escapes the content value,
preventing SSTI.

Chain broken: content is context value -> static template -> no SSTI
"""
from flask import Blueprint, request, jsonify
from renderer import render_page

page_bp = Blueprint("page", __name__)


# vuln-code-snippet start chain_dynamic_page_safe
@page_bp.route("/page", methods=["POST"])
def page():
    """Render a CMS page with content treated as a safe escaped variable."""
    content = request.json.get("content", "")
    context = request.json.get("context", {})
    output = render_page(content, context)  # vuln-code-snippet safe-line chain_dynamic_page_safe
    return jsonify({"html": output})
# vuln-code-snippet end chain_dynamic_page_safe
