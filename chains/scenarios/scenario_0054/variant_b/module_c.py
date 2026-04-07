from flask import Blueprint, request, make_response
from module_b import cache_key, get_cached, set_cached

views_bp = Blueprint("views", __name__)

@views_bp.route("/profile")
def profile():
    key = cache_key()
    cached = get_cached(key)
    if cached:
        return cached
    user = request.cookies.get("user", "guest")
    resp = make_response(f"<p>Hello {user}</p>")
    resp.headers["Cache-Control"] = "private, no-store"
    set_cached(key, resp)
    return resp
