"""CORS and CSRF configuration -- SAFE variant.

apply_cors_headers() only sets the CORS Origin header for explicitly
allowed origins.  An unknown origin receives no ACAO header, so the browser
blocks the cross-origin response and the preflight fails for credentialed requests.

Chain: cross-origin page -> preflight rejected (origin not whitelisted) -> blocked
"""
from flask import request

ALLOWED_ORIGINS = {"https://app.example.com", "https://www.example.com"}


def apply_cors_headers(response):
    """Set CORS headers only for whitelisted origins."""
    origin = request.headers.get("Origin", "")
    if origin in ALLOWED_ORIGINS:
        response.headers["Access-Control-Allow-Origin"] = origin
        response.headers["Access-Control-Allow-Credentials"] = "true"
        response.headers["Access-Control-Allow-Methods"] = "GET, POST, OPTIONS"
        response.headers["Access-Control-Allow-Headers"] = "Content-Type"
    return response
