"""Views -- VULNERABLE variant for cookie_in_cache_key.

GET /profile returns a response cached with a key that includes the
Cookie header value.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Blueprint, request, make_response
from cache import cache_key, get_cached, set_cached

views_bp = Blueprint("views", __name__)


# vuln-code-snippet start chain_cookie_cache_vuln
@views_bp.route("/profile")
def profile():
    """Return a cached profile page keyed on cookie."""
    key = cache_key()
    cached = get_cached(key)
    if cached:
        return cached
    user = request.cookies.get("user", "guest")
    resp = make_response(f"<p>Hello {user}</p>")
    resp.headers["Cache-Control"] = "public, max-age=300"
    set_cached(key, resp)  # vuln-code-snippet vuln-line chain_cookie_cache_vuln
    return resp
# vuln-code-snippet end chain_cookie_cache_vuln
