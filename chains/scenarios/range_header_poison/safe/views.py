"""Views -- SAFE variant for range_header_poison.

GET /resource serves byte ranges but never caches partial (206) responses.

This file is IDENTICAL between vuln/ and safe/ variants (only
cache.py changes).
"""
from flask import Blueprint, request, make_response
from cache import cache_key_for, get_cached, set_cached

views_bp = Blueprint("views", __name__)

_RESOURCE = b"A" * 1024


# vuln-code-snippet start chain_range_poison_safe
@views_bp.route("/resource")
def resource():
    """Serve a resource; only full responses enter the cache."""
    key = cache_key_for(request.path)
    cached = get_cached(key)
    if cached:
        return cached
    range_header = request.headers.get("Range")
    if range_header and range_header.startswith("bytes="):
        parts = range_header[6:].split("-")
        start = int(parts[0]) if parts[0] else 0
        end = int(parts[1]) if len(parts) > 1 and parts[1] else len(_RESOURCE) - 1
        data = _RESOURCE[start:end + 1]
        resp = make_response(data, 206)
        resp.headers["Content-Range"] = f"bytes {start}-{end}/{len(_RESOURCE)}"
        set_cached(key, resp, is_partial=True)  # vuln-code-snippet safe-line chain_range_poison_safe
        return resp
    resp = make_response(_RESOURCE)
    resp.headers["Cache-Control"] = "public, max-age=300"
    set_cached(key, resp, is_partial=False)
    return resp
# vuln-code-snippet end chain_range_poison_safe
