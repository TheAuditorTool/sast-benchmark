"""CORS middleware -- SAFE variant.

Uses a specific whitelisted origin instead of a wildcard. Credentials are
only allowed for that known origin. This conforms to the CORS specification
and does not expose the API to all origins.

Chain broken: only specific origin allowed -> browser enforces -> cross-origin reads blocked
"""
from flask import request

ALLOWED_ORIGIN = "https://app.example.com"


def apply_cors(response):
    """Add CORS headers only for the whitelisted origin."""
    origin = request.headers.get("Origin", "")
    if origin == ALLOWED_ORIGIN:
        response.headers["Access-Control-Allow-Origin"] = ALLOWED_ORIGIN
        response.headers["Access-Control-Allow-Credentials"] = "true"
        response.headers["Access-Control-Allow-Methods"] = "GET, POST, OPTIONS"
        response.headers["Access-Control-Allow-Headers"] = "Content-Type, Authorization"
    return response
