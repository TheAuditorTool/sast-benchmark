"""Views -- SAFE variant for x_forwarded_host_poison.

GET /asset returns a cached page whose canonical link is built from the
trusted hard-coded origin.  X-Forwarded-Host is ignored.

This file is IDENTICAL between vuln/ and safe/ variants (only
response_builder.py changes).
"""
from flask import Blueprint, make_response
from response_builder import build_url

views_bp = Blueprint("views", __name__)


# vuln-code-snippet start chain_x_fwd_host_safe
@views_bp.route("/asset")
def asset():
    """Return a cacheable page with a canonical link."""
    canonical = build_url("/asset")
    html = f'<html><head><link rel="canonical" href="{canonical}"/></head><body>Asset</body></html>'
    resp = make_response(html)
    resp.headers["Cache-Control"] = "public, max-age=600"
    return resp  # vuln-code-snippet safe-line chain_x_fwd_host_safe
# vuln-code-snippet end chain_x_fwd_host_safe
