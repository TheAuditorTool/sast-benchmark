"""Views -- VULNERABLE variant for link_header_injection.

GET /page?res=<url> returns a cached page with a Link preload header
for the supplied resource URL.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Blueprint, request, make_response
from response_builder import build_link_header

views_bp = Blueprint("views", __name__)


# vuln-code-snippet start chain_link_header_vuln
@views_bp.route("/page")
def page():
    """Return a cached page with a Link preload header."""
    resource = request.args.get("res", "/static/app.js")
    resp = make_response("<html><body>Page</body></html>")
    resp.headers["Link"] = build_link_header(resource)  # vuln-code-snippet vuln-line chain_link_header_vuln
    resp.headers["Cache-Control"] = "public, max-age=300"
    return resp
# vuln-code-snippet end chain_link_header_vuln
