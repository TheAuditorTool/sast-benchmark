"""CORS middleware -- SAFE variant.

Restricts CORS access to the GraphQL endpoint to a single whitelisted origin.
Arbitrary websites cannot make credentialed GraphQL queries, preventing
cross-origin schema discovery and data exfiltration.

Chain broken: only one origin allowed -> attacker origin rejected -> GraphQL inaccessible cross-origin
"""
from flask import request

ALLOWED_ORIGIN = "https://app.example.com"


def apply_cors(response):
    """Add CORS headers only for the whitelisted GraphQL client origin."""
    origin = request.headers.get("Origin", "")
    if origin == ALLOWED_ORIGIN:
        response.headers["Access-Control-Allow-Origin"] = ALLOWED_ORIGIN
        response.headers["Access-Control-Allow-Credentials"] = "true"
        response.headers["Access-Control-Allow-Methods"] = "GET, POST, OPTIONS"
        response.headers["Access-Control-Allow-Headers"] = "Content-Type, Authorization"
    return response
