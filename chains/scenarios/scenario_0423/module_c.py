from flask import Blueprint, request
from module_b import build_redirect

views_bp = Blueprint("views", __name__)

@views_bp.route("/redirect")
def do_redirect():
    url = request.args.get("url", "/")
    return build_redirect(url)
