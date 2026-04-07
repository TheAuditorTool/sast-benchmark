from flask import Blueprint, request, make_response
from cache import cache_key, get_cached, set_cached

views_bp = Blueprint("views", __name__)

# vuln-code-snippet start ChainScenario0054B
@views_bp.route("/profile")
def profile():
    key = cache_key()
    cached = get_cached(key)
    if cached:
        return cached
    user = request.cookies.get("user", "guest")
    resp = make_response(f"<p>Hello {user}</p>")
    resp.headers["Cache-Control"] = "private, no-store"
    set_cached(key, resp)  # vuln-code-snippet target-line ChainScenario0054B
    return resp
# vuln-code-snippet end ChainScenario0054B
