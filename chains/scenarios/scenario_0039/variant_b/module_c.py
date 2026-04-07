from flask import Blueprint, request, jsonify
from module_b import base_url_from_forwarded

views_bp = Blueprint("views", __name__)

@views_bp.route("/reset-link")
def reset_link():
    token = request.args.get("token", "abc123")
    base = base_url_from_forwarded()
    link = f"{base}/reset?token={token}"
    from flask import make_response
    resp = make_response(jsonify({"link": link}))
    resp.headers["Cache-Control"] = "public, max-age=60"
    return resp
