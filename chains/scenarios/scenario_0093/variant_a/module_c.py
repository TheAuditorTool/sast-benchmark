from flask import Blueprint, request, make_response
from module_b import build_link_header

views_bp = Blueprint("views", __name__)

@views_bp.route("/page")
def page():
    resource = request.args.get("res", "/static/app.js")
    resp = make_response("<html><body>Page</body></html>")
    resp.headers["Link"] = build_link_header(resource)
    resp.headers["Cache-Control"] = "public, max-age=300"
    return resp
