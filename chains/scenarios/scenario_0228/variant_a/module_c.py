from flask import Blueprint, request
from module_b import build_response

views_bp = Blueprint("views", __name__)

_FILES = {
    "report": b"Annual report data",
    "logo": b"<binary image data>",
}

@views_bp.route("/file")
def serve_file():
    name = request.args.get("name", "report")
    body = _FILES.get(name, b"Not found")
    return build_response(name, body)
