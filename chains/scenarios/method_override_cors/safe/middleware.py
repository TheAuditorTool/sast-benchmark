"""CORS + method-override middleware -- SAFE variant.

The server ignores the X-HTTP-Method-Override header entirely. Destructive
operations (DELETE) require a proper DELETE request, which triggers a CORS
preflight. The CORS policy also restricts origins to an allowlist, so the
preflight itself must come from a trusted origin.

Chain broken: no method override honored -> DELETE requires preflight -> only trusted origin can preflight
"""
from flask import request

ALLOWED_ORIGINS = {"https://app.example.com"}


def get_effective_method() -> str:
    """Return the actual HTTP method; X-HTTP-Method-Override is ignored."""
    return request.method


def apply_cors(response):
    """Add CORS headers for allowed origins requiring preflight for non-simple methods."""
    origin = request.headers.get("Origin", "")
    if origin in ALLOWED_ORIGINS:
        response.headers["Access-Control-Allow-Origin"] = origin
        response.headers["Access-Control-Allow-Credentials"] = "true"
        response.headers["Access-Control-Allow-Methods"] = "GET, DELETE, OPTIONS"
    return response
