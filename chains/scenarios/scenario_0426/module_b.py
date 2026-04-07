from flask import Blueprint

response_builder_bp = Blueprint("response_builder", __name__)

def build_redirect(location: str):
    from flask import make_response
    resp = make_response("", 302)
    resp.headers["Location"] = location  
    resp.headers["Cache-Control"] = "public, max-age=300"
    return resp
