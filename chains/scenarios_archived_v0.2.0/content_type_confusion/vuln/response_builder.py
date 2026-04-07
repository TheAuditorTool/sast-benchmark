"""Response builder -- VULNERABLE variant.

Derives the Content-Type from the filename extension supplied by the
caller without validation, and caches the response publicly.  An attacker
can request a path ending in .js or .html to override the Content-Type
of a resource, triggering MIME sniffing or XSS when the cached version
is served to future visitors.

Chain: attacker-chosen filename extension -> wrong Content-Type cached ->
       browser sniffs/executes content -> XSS or script injection.
Individual findings: incorrect Content-Type (low)
Chain finding: Content-Type confusion -> MIME-sniff XSS via cache (high)
"""
import mimetypes
from flask import Blueprint

response_builder_bp = Blueprint("response_builder", __name__)


def build_response(filename: str, body: bytes):
    """Return a response whose Content-Type is guessed from filename."""
    from flask import make_response
    content_type, _ = mimetypes.guess_type(filename)
    if not content_type:
        content_type = "application/octet-stream"
    resp = make_response(body)
    resp.headers["Content-Type"] = content_type
    resp.headers["Cache-Control"] = "public, max-age=600"
    return resp
