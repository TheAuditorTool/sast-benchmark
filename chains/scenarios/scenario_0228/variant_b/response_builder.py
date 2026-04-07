from flask import Blueprint

response_builder_bp = Blueprint("response_builder", __name__)

def build_response(filename: str, body: bytes):
    from flask import make_response
    resp = make_response(body)
    resp.headers["Content-Type"] = "application/octet-stream"
    resp.headers["X-Content-Type-Options"] = "nosniff"
    resp.headers["Cache-Control"] = "public, max-age=600"
    return resp
