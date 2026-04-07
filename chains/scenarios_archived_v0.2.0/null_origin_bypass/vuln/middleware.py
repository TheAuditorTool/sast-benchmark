"""CORS middleware -- VULNERABLE variant.

Allows Origin: null in addition to the legitimate origin. Browsers send
Origin: null for sandboxed iframes and local file:// requests. An attacker
can embed a sandboxed iframe that sends credentialed requests with Origin:
null, which this policy accepts, enabling cross-origin data theft.

Chain: null origin allowed + credentials -> sandboxed iframe reads authenticated response -> data stolen
Individual findings: CORS policy allows null origin (CWE-942)
Chain finding: cross-origin data theft via null-origin CORS bypass (high)
"""
from flask import request

ALLOWED_ORIGINS = {"https://app.example.com", "null"}


def apply_cors(response):
    """Add CORS headers, accepting null as a valid origin."""
    origin = request.headers.get("Origin", "")
    if origin in ALLOWED_ORIGINS:
        response.headers["Access-Control-Allow-Origin"] = origin
        response.headers["Access-Control-Allow-Credentials"] = "true"
        response.headers["Access-Control-Allow-Methods"] = "GET, POST, OPTIONS"
    return response
