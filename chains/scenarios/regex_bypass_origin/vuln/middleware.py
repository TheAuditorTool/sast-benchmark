"""CORS middleware -- VULNERABLE variant.

Validates the Origin header with a regex that matches 'example.com' anywhere
in the string. The unanchored pattern allows origins such as
'https://evil-example.com' or 'https://example.com.evil.io' to match,
granting those sites full credentialed cross-origin access.

Chain: bad regex matches evil-example.com -> CORS granted with credentials -> attacker reads response
Individual findings: CORS origin validated by unanchored regex (CWE-942)
Chain finding: cross-origin data theft via regex bypass (high)
"""
import re
from flask import request

_ORIGIN_RE = re.compile(r"example\.com")


def apply_cors(response):
    """Add CORS headers when origin matches the (too-broad) regex."""
    origin = request.headers.get("Origin", "")
    if origin and _ORIGIN_RE.search(origin):
        response.headers["Access-Control-Allow-Origin"] = origin
        response.headers["Access-Control-Allow-Credentials"] = "true"
        response.headers["Access-Control-Allow-Methods"] = "GET, POST, OPTIONS"
    return response
