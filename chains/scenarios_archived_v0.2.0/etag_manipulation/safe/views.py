"""Views -- SAFE variant for etag_manipulation.

GET /resource serves content with conditional GET support that validates
the ETag against the server-authoritative value.

This file is IDENTICAL between vuln/ and safe/ variants (only
cache.py changes).
"""
from flask import Blueprint, make_response
from cache import handle_conditional, RESOURCE_BODY

views_bp = Blueprint("views", __name__)


# vuln-code-snippet start chain_etag_manip_safe
@views_bp.route("/resource")
def resource():
    """Serve a resource with validated conditional GET support."""
    resp = make_response(RESOURCE_BODY)
    resp.headers["Cache-Control"] = "public, max-age=300"
    return handle_conditional(resp)  # vuln-code-snippet safe-line chain_etag_manip_safe
# vuln-code-snippet end chain_etag_manip_safe
