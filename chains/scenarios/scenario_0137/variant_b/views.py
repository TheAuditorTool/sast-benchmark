from flask import Blueprint, request, make_response
from cache import get_cached, set_cached, cache_key

views_bp = Blueprint("views", __name__)

# vuln-code-snippet start ChainScenario0137B
@views_bp.route("/data", methods=["GET"])
@views_bp.route("/DATA", methods=["GET"])
def data():
    key = cache_key()
    cached = get_cached(key)
    if cached:
        return cached
    content = f"<p>Data for {request.path.lower()}</p>"
    resp = make_response(content)
    resp.headers["Cache-Control"] = "public, max-age=300"
    set_cached(key, resp)  # vuln-code-snippet target-line ChainScenario0137B
    return resp
# vuln-code-snippet end ChainScenario0137B
