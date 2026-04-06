"""CORS and CSRF configuration -- VULNERABLE variant.

The CORS policy reflects any Origin for /api/* endpoints.  While CORS
prevents browsers from reading cross-origin responses, it does NOT prevent
the request from being made.  Combined with cookie authentication and no
CSRF token, a cross-origin page can still trigger state changes.

Chain: cross-origin page -> CORS reflected -> POST /api/settings accepted -> state changed
"""
from flask import request


def apply_cors_headers(response):
    """Reflect caller's Origin in CORS headers -- this is the vulnerable pattern."""
    origin = request.headers.get("Origin", "")
    if origin:
        response.headers["Access-Control-Allow-Origin"] = origin  # VULN: reflects any origin
        response.headers["Access-Control-Allow-Credentials"] = "true"
        response.headers["Access-Control-Allow-Methods"] = "GET, POST, OPTIONS"
        response.headers["Access-Control-Allow-Headers"] = "Content-Type"
    return response
