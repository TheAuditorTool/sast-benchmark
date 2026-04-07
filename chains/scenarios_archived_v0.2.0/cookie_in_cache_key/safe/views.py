"""Views -- SAFE variant for cookie_in_cache_key.

GET /profile returns a non-cached (private) personalised response so
cookie-based cache confusion is impossible.

This file is IDENTICAL between vuln/ and safe/ variants (only
cache.py changes).
"""
from flask import Blueprint, request, make_response
from cache import cache_key, get_cached, set_cached

views_bp = Blueprint("views", __name__)


# vuln-code-snippet start chain_cookie_cache_safe
@views_bp.route("/profile")
def profile():
    """Return a private profile page (not cached publicly)."""
    key = cache_key()
    cached = get_cached(key)
    if cached:
        return cached
    user = request.cookies.get("user", "guest")
    resp = make_response(f"<p>Hello {user}</p>")
    resp.headers["Cache-Control"] = "private, no-store"
    set_cached(key, resp)  # vuln-code-snippet safe-line chain_cookie_cache_safe
    return resp
# vuln-code-snippet end chain_cookie_cache_safe
