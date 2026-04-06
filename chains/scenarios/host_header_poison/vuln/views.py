"""Views -- VULNERABLE variant for host_header_poison.

GET /home returns an HTML page with navigation links built from the
Host header.  The response is marked public/cacheable so poisoned
content can be served to future visitors.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Blueprint, make_response
from response_builder import build_base_url

views_bp = Blueprint("views", __name__)


# vuln-code-snippet start chain_host_header_poison_vuln
@views_bp.route("/home")
def home():
    """Return a cacheable home page with absolute navigation links."""
    base = build_base_url()
    html = f"<html><body><a href=\"{base}/profile\">Profile</a></body></html>"
    resp = make_response(html)
    resp.headers["Cache-Control"] = "public, max-age=600"
    return resp  # vuln-code-snippet vuln-line chain_host_header_poison_vuln
# vuln-code-snippet end chain_host_header_poison_vuln
