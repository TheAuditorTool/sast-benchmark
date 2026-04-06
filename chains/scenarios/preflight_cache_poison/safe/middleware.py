"""CORS middleware -- SAFE variant.

Sets Access-Control-Max-Age to 600 seconds (10 minutes) and uses a strict
origin allowlist. The short cache TTL means browsers re-check permissions
frequently, limiting the window during which a poisoned cache could persist.

Chain broken: short preflight TTL -> browser re-checks frequently -> policy changes take effect quickly
"""
from flask import request

ALLOWED_ORIGINS = {"https://app.example.com"}


def apply_cors(response):
    """Add CORS headers with a short preflight cache duration."""
    origin = request.headers.get("Origin", "")
    if origin in ALLOWED_ORIGINS:
        response.headers["Access-Control-Allow-Origin"] = origin
        response.headers["Access-Control-Allow-Credentials"] = "true"
        response.headers["Access-Control-Allow-Methods"] = "GET, POST, DELETE, OPTIONS"
        response.headers["Access-Control-Allow-Headers"] = "Content-Type, Authorization"
        response.headers["Access-Control-Max-Age"] = "600"
    return response
