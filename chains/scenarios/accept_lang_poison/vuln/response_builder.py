"""Response builder -- VULNERABLE variant.

Reflects the Accept-Language header value verbatim into the response
body and caches without a Vary header.  An attacker can inject a
script payload via Accept-Language that gets stored in the cache and
served to subsequent visitors.

Chain: attacker-crafted Accept-Language -> reflected in body ->
       cached without Vary -> poisoned response served to all visitors.
Individual findings: reflected header value (low), missing Vary (info)
Chain finding: Accept-Language reflection -> cache poisoning -> XSS (high)
"""
from flask import Blueprint, request

response_builder_bp = Blueprint("response_builder", __name__)


def build_lang_response() -> str:
    """Return HTML body containing the raw Accept-Language value."""
    lang = request.headers.get("Accept-Language", "en")
    return f"<html><body><p>Language: {lang}</p></body></html>"
