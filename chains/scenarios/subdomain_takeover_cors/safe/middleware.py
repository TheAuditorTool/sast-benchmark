"""CORS middleware -- SAFE variant.

Uses an explicit allowlist of known, active origins rather than accepting
any subdomain. Even if an attacker takes over an unused subdomain of
example.com, it will not appear in the allowlist and will be rejected.

Chain broken: only explicit origins allowed -> taken-over subdomain rejected -> data protected
"""
from flask import request

ALLOWED_ORIGINS = {
    "https://app.example.com",
    "https://admin.example.com",
}


def apply_cors(response):
    """Add CORS headers only for explicitly listed origins."""
    origin = request.headers.get("Origin", "")
    if origin in ALLOWED_ORIGINS:
        response.headers["Access-Control-Allow-Origin"] = origin
        response.headers["Access-Control-Allow-Credentials"] = "true"
        response.headers["Access-Control-Allow-Methods"] = "GET, POST, OPTIONS"
    return response
