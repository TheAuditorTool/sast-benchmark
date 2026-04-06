"""CORS middleware -- SAFE variant.

Validates the Origin header against a fully anchored regex that matches
only the exact expected origin. The pattern requires the string to start
with 'https://app.example.com' and end there, so domains such as
'evil-example.com' cannot match.

Chain broken: anchored regex rejects evil domains -> CORS not granted -> browser blocks read
"""
import re
from flask import request

_ORIGIN_RE = re.compile(r"^https://app\.example\.com$")


def apply_cors(response):
    """Add CORS headers only when the origin matches the anchored regex."""
    origin = request.headers.get("Origin", "")
    if origin and _ORIGIN_RE.match(origin):
        response.headers["Access-Control-Allow-Origin"] = origin
        response.headers["Access-Control-Allow-Credentials"] = "true"
        response.headers["Access-Control-Allow-Methods"] = "GET, POST, OPTIONS"
    return response
