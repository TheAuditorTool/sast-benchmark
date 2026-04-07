from flask import Blueprint

response_builder_bp = Blueprint("response_builder", __name__)

def build_redirect(location: str):
    from flask import make_response
    safe_location = location.replace("\r", "").replace("\n", "")
    resp = make_response("", 302)
    resp.headers["Location"] = safe_location
    resp.headers["Cache-Control"] = "public, max-age=300"
    return resp
