"""Response builder -- SAFE variant.

Strips CR and LF from the Location value before setting the header,
breaking the CRLF injection vector and preventing cache poisoning.

Chain broken: CRLF stripped -> no header injection -> cache is clean
"""
from flask import Blueprint

response_builder_bp = Blueprint("response_builder", __name__)


def build_redirect(location: str):
    """Return a 302 response with sanitised Location value."""
    from flask import make_response
    safe_location = location.replace("\r", "").replace("\n", "")
    resp = make_response("", 302)
    resp.headers["Location"] = safe_location
    resp.headers["Cache-Control"] = "public, max-age=300"
    return resp
