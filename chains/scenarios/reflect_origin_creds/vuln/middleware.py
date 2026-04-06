"""CORS middleware -- VULNERABLE variant.

Reflects the caller's Origin header back as Access-Control-Allow-Origin and
simultaneously sets Access-Control-Allow-Credentials: true. This combination
means any website can make credentialed cross-origin requests (sending cookies
and authorization headers) and read the response, enabling session hijacking
and data theft.

Chain: arbitrary Origin reflected + credentials allowed -> any site reads authenticated responses
Individual findings: CORS origin reflection with credentials (CWE-942)
Chain finding: cross-origin data theft via reflected-origin CORS policy (critical)
"""
from flask import request


def apply_cors(response):
    """Add CORS headers that reflect the incoming Origin."""
    origin = request.headers.get("Origin", "")
    if origin:
        response.headers["Access-Control-Allow-Origin"] = origin
        response.headers["Access-Control-Allow-Credentials"] = "true"
        response.headers["Access-Control-Allow-Methods"] = "GET, POST, OPTIONS"
        response.headers["Access-Control-Allow-Headers"] = "Content-Type, Authorization"
    return response
