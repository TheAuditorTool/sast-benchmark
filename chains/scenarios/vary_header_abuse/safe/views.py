"""Views -- SAFE variant for vary_header_abuse.

GET /page returns localised content and caches it with Vary:
Accept-Language so each language variant has its own cache entry.

This file is IDENTICAL between vuln/ and safe/ variants (only
cache.py changes).
"""
from flask import Blueprint, request, make_response
from cache import apply_cache_headers

views_bp = Blueprint("views", __name__)

TRANSLATIONS = {
    "fr": "Bonjour le monde",
    "de": "Hallo Welt",
}


# vuln-code-snippet start chain_vary_header_safe
@views_bp.route("/page")
def page():
    """Return localised content with a correct Vary header."""
    lang = request.headers.get("Accept-Language", "en")[:2]
    body = TRANSLATIONS.get(lang, "Hello world")
    resp = make_response(f"<p>{body}</p>")
    resp = apply_cache_headers(resp)  # vuln-code-snippet safe-line chain_vary_header_safe
    return resp
# vuln-code-snippet end chain_vary_header_safe
