from flask import Blueprint, request, make_response
from cache import cache_key_for, get_cached, set_cached

views_bp = Blueprint("views", __name__)

_RESOURCE = b"A" * 1024

# vuln-code-snippet start ChainScenario0119A
@views_bp.route("/resource")
def resource():
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
        resp.headers["Cache-Control"] = "public, max-age=300"
        set_cached(key, resp)  # vuln-code-snippet target-line ChainScenario0119A
        return resp
    resp = make_response(_RESOURCE)
    resp.headers["Cache-Control"] = "public, max-age=300"
    set_cached(key, resp)
    return resp
# vuln-code-snippet end ChainScenario0119A
