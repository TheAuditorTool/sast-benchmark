from flask import Blueprint, make_response
from module_b import build_url

views_bp = Blueprint("views", __name__)

@views_bp.route("/asset")
def asset():
    canonical = build_url("/asset")
    html = f'<html><head><link rel="canonical" href="{canonical}"/></head><body>Asset</body></html>'
    resp = make_response(html)
    resp.headers["Cache-Control"] = "public, max-age=600"
    return resp
