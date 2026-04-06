"""CORS middleware -- SAFE variant.

Only sets Access-Control-Allow-Origin for explicitly whitelisted origins.
Access-Control-Allow-Credentials is only sent when the origin is trusted.
Unknown origins receive no CORS headers, so cross-origin reads are blocked
by the browser's same-origin policy.

Chain broken: unknown origins get no CORS header -> browser blocks cross-origin reads -> data protected
"""
from flask import request

ALLOWED_ORIGINS = {"https://app.example.com", "https://admin.example.com"}


def apply_cors(response):
    """Add CORS headers only for whitelisted origins."""
    origin = request.headers.get("Origin", "")
    if origin in ALLOWED_ORIGINS:
        response.headers["Access-Control-Allow-Origin"] = origin
        response.headers["Access-Control-Allow-Credentials"] = "true"
        response.headers["Access-Control-Allow-Methods"] = "GET, POST, OPTIONS"
        response.headers["Access-Control-Allow-Headers"] = "Content-Type, Authorization"
    return response
