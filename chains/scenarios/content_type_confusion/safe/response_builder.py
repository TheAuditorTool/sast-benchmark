"""Response builder -- SAFE variant.

Uses a fixed Content-Type regardless of the filename, and adds
X-Content-Type-Options: nosniff to prevent the browser from
overriding it.

Chain broken: Content-Type is not attacker-influenced -> no MIME confusion
              -> cached response is safe to serve
"""
from flask import Blueprint

response_builder_bp = Blueprint("response_builder", __name__)


def build_response(filename: str, body: bytes):
    """Return a response with a fixed, safe Content-Type."""
    from flask import make_response
    resp = make_response(body)
    resp.headers["Content-Type"] = "application/octet-stream"
    resp.headers["X-Content-Type-Options"] = "nosniff"
    resp.headers["Cache-Control"] = "public, max-age=600"
    return resp
