"""Response builder -- VULNERABLE variant.

Builds redirect responses using a caller-supplied Location value without
stripping CRLF characters.  An attacker who controls the redirect target
can inject arbitrary headers (and a fake body) into the response.

Chain: CRLF in Location header -> injected headers stored in cache ->
       poisoned response served to future visitors (cache-poisoned XSS).
Individual findings: CRLF / header injection (high)
Chain finding: CRLF injection -> cache poisoning -> stored XSS (critical)
"""
from flask import Blueprint

response_builder_bp = Blueprint("response_builder", __name__)


def build_redirect(location: str):
    """Return a 302 response with caller-supplied Location value."""
    from flask import make_response
    resp = make_response("", 302)
    resp.headers["Location"] = location  # CRLF characters passed through
    resp.headers["Cache-Control"] = "public, max-age=300"
    return resp
