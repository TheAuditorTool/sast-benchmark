"""Views -- VULNERABLE variant for accept_lang_poison.

GET /lang returns Accept-Language-based content cached publicly without
a Vary header.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Blueprint, make_response
from response_builder import build_lang_response

views_bp = Blueprint("views", __name__)


# vuln-code-snippet start chain_accept_lang_poison_vuln
@views_bp.route("/lang")
def lang():
    """Return language-specific content (cached)."""
    body = build_lang_response()
    resp = make_response(body)
    resp.headers["Cache-Control"] = "public, max-age=300"
    return resp  # vuln-code-snippet vuln-line chain_accept_lang_poison_vuln
# vuln-code-snippet end chain_accept_lang_poison_vuln
