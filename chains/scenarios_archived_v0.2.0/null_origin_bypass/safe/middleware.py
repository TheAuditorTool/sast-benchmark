"""CORS middleware -- SAFE variant.

Only allows explicitly named HTTPS origins. The literal string "null" is
not in the allowlist, so sandboxed iframe attacks using Origin: null are
rejected. The browser will block cross-origin reads from such frames.

Chain broken: null origin not in allowlist -> CORS header not sent -> browser blocks iframe read
"""
from flask import request

ALLOWED_ORIGINS = {"https://app.example.com"}


def apply_cors(response):
    """Add CORS headers only for whitelisted HTTPS origins."""
    origin = request.headers.get("Origin", "")
    if origin in ALLOWED_ORIGINS:
        response.headers["Access-Control-Allow-Origin"] = origin
        response.headers["Access-Control-Allow-Credentials"] = "true"
        response.headers["Access-Control-Allow-Methods"] = "GET, POST, OPTIONS"
    return response
