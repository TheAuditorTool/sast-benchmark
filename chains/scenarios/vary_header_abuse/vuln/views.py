"""Views -- VULNERABLE variant for vary_header_abuse.

GET /page returns localised content based on Accept-Language and marks
it cacheable without Vary, enabling cache poisoning.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Blueprint, request, make_response
from cache import apply_cache_headers

views_bp = Blueprint("views", __name__)

TRANSLATIONS = {
    "fr": "Bonjour le monde",
    "de": "Hallo Welt",
}


# vuln-code-snippet start chain_vary_header_vuln
@views_bp.route("/page")
def page():
    """Return localised content without a Vary header."""
    lang = request.headers.get("Accept-Language", "en")[:2]
    body = TRANSLATIONS.get(lang, "Hello world")
    resp = make_response(f"<p>{body}</p>")
    resp = apply_cache_headers(resp)  # vuln-code-snippet vuln-line chain_vary_header_vuln
    return resp
# vuln-code-snippet end chain_vary_header_vuln
