"""Views -- VULNERABLE variant for cache_key_normalize.

GET /data normalises the path when generating the response but stores
and retrieves from the cache using the raw (unnormalised) key.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Blueprint, request, make_response
from cache import get_cached, set_cached, cache_key

views_bp = Blueprint("views", __name__)


# vuln-code-snippet start chain_cache_key_vuln
@views_bp.route("/data", methods=["GET"])
@views_bp.route("/DATA", methods=["GET"])
def data():
    """Serve data, caching by raw path key."""
    key = cache_key()
    cached = get_cached(key)
    if cached:
        return cached
    content = f"<p>Data for {request.path.lower()}</p>"
    resp = make_response(content)
    resp.headers["Cache-Control"] = "public, max-age=300"
    set_cached(key, resp)  # vuln-code-snippet vuln-line chain_cache_key_vuln
    return resp
# vuln-code-snippet end chain_cache_key_vuln
