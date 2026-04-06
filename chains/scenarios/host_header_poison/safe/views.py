"""Views -- SAFE variant for host_header_poison.

GET /home returns an HTML page with navigation links built from the
trusted hard-coded origin.  An attacker-supplied Host header is ignored.

This file is IDENTICAL between vuln/ and safe/ variants (only
response_builder.py changes).
"""
from flask import Blueprint, make_response
from response_builder import build_base_url

views_bp = Blueprint("views", __name__)


# vuln-code-snippet start chain_host_header_poison_safe
@views_bp.route("/home")
def home():
    """Return a cacheable home page with absolute navigation links."""
    base = build_base_url()
    html = f"<html><body><a href=\"{base}/profile\">Profile</a></body></html>"
    resp = make_response(html)
    resp.headers["Cache-Control"] = "public, max-age=600"
    return resp  # vuln-code-snippet safe-line chain_host_header_poison_safe
# vuln-code-snippet end chain_host_header_poison_safe
