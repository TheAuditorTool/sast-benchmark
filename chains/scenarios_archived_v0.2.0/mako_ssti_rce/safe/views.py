"""Flask views for mako_ssti_rce scenario -- SAFE variant.

POST /render passes the client-supplied content as a context variable
to renderer.render_page. The Mako template source is static and the
h filter escapes the value, preventing injection.

Chain broken: content is a context value -> static template -> no RCE
"""
from flask import Blueprint, request, jsonify
from renderer import render_page

render_bp = Blueprint("render", __name__)


# vuln-code-snippet start chain_mako_ssti_safe
@render_bp.route("/render", methods=["POST"])
def render():
    """Render client content using a static Mako template."""
    content = request.json.get("template", "")
    output = render_page(content)  # vuln-code-snippet safe-line chain_mako_ssti_safe
    return jsonify({"output": output})
# vuln-code-snippet end chain_mako_ssti_safe
