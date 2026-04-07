"""CORS middleware -- SAFE variant.

Adds Vary: Origin alongside the Access-Control-Allow-Origin header.
This instructs shared caches to store separate response variants per
origin, preventing one origin's CORS header from being served to another.

Chain broken: Vary: Origin present -> cache stores per-origin responses -> ACAO headers not confused
"""
from flask import request

ALLOWED_ORIGINS = {"https://app.example.com", "https://partner.example.com"}


def apply_cors(response):
    """Add CORS headers including the required Vary: Origin directive."""
    origin = request.headers.get("Origin", "")
    response.headers.add("Vary", "Origin")
    if origin in ALLOWED_ORIGINS:
        response.headers["Access-Control-Allow-Origin"] = origin
        response.headers["Access-Control-Allow-Credentials"] = "true"
    return response
