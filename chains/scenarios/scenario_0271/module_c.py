from flask import Blueprint, make_response
from module_b import build_base_url

views_bp = Blueprint("views", __name__)

@views_bp.route("/home")
def home():
    base = build_base_url()
    html = f"<html><body><a href=\"{base}/profile\">Profile</a></body></html>"
    resp = make_response(html)
    resp.headers["Cache-Control"] = "public, max-age=600"
    return resp
