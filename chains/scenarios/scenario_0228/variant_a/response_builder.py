import mimetypes
from flask import Blueprint

response_builder_bp = Blueprint("response_builder", __name__)

def build_response(filename: str, body: bytes):
    from flask import make_response
    content_type, _ = mimetypes.guess_type(filename)
    if not content_type:
        content_type = "application/octet-stream"
    resp = make_response(body)
    resp.headers["Content-Type"] = content_type
    resp.headers["Cache-Control"] = "public, max-age=600"
    return resp
