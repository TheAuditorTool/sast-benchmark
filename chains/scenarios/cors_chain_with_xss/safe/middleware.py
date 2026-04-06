"""CORS middleware -- SAFE variant.

Removes feedback.example.com from the allowlist, restricting CORS access
to the main application origin only. Even if the feedback subdomain has an
XSS vulnerability, it cannot make credentialed requests to this API.

Chain broken: vulnerable subdomain removed from allowlist -> XSS script at feedback cannot read API -> data protected
"""
from flask import request

ALLOWED_ORIGINS = {
    "https://app.example.com",
}


def apply_cors(response):
    """Add CORS headers only for the primary application origin."""
    origin = request.headers.get("Origin", "")
    if origin in ALLOWED_ORIGINS:
        response.headers["Access-Control-Allow-Origin"] = origin
        response.headers["Access-Control-Allow-Credentials"] = "true"
        response.headers["Access-Control-Allow-Methods"] = "GET, POST, OPTIONS"
    return response
