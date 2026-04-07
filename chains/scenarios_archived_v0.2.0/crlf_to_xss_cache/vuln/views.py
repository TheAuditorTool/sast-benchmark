"""Views -- VULNERABLE variant for crlf_to_xss_cache.

GET /redirect?url=<value> issues a cached redirect to the supplied URL.
Because response_builder.build_redirect does not strip CRLF, an attacker
can inject headers and a body that gets stored in the cache.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Blueprint, request
from response_builder import build_redirect

views_bp = Blueprint("views", __name__)


# vuln-code-snippet start chain_crlf_xss_cache_vuln
@views_bp.route("/redirect")
def do_redirect():
    """Issue a cached redirect to the caller-supplied URL."""
    url = request.args.get("url", "/")
    return build_redirect(url)  # vuln-code-snippet vuln-line chain_crlf_xss_cache_vuln
# vuln-code-snippet end chain_crlf_xss_cache_vuln
