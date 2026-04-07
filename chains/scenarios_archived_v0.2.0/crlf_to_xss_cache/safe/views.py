"""Views -- SAFE variant for crlf_to_xss_cache.

GET /redirect?url=<value> issues a cached redirect to the supplied URL.
response_builder.build_redirect strips CRLF before setting the header,
so injected headers cannot enter the cache.

This file is IDENTICAL between vuln/ and safe/ variants (only
response_builder.py changes).
"""
from flask import Blueprint, request
from response_builder import build_redirect

views_bp = Blueprint("views", __name__)


# vuln-code-snippet start chain_crlf_xss_cache_safe
@views_bp.route("/redirect")
def do_redirect():
    """Issue a cached redirect to the caller-supplied URL."""
    url = request.args.get("url", "/")
    return build_redirect(url)  # vuln-code-snippet safe-line chain_crlf_xss_cache_safe
# vuln-code-snippet end chain_crlf_xss_cache_safe
