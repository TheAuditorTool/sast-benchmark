from flask import Blueprint, make_response
from module_b import build_lang_response

views_bp = Blueprint("views", __name__)

@views_bp.route("/lang")
def lang():
    body = build_lang_response()
    resp = make_response(body)
    resp.headers["Cache-Control"] = "public, max-age=300"
    return resp
