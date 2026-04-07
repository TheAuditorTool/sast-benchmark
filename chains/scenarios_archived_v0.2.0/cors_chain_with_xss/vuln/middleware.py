"""CORS middleware -- VULNERABLE variant.

Trusts feedback.example.com as an allowed CORS origin. That subdomain hosts
a user-feedback widget that contains a stored XSS vulnerability. An attacker
who exploits the XSS can make credentialed requests to this API from the
trusted origin and read the responses, because the CORS policy trusts it.

Chain: XSS on trusted subdomain -> script runs at trusted origin -> reads authenticated API response
Individual findings: CORS trusts XSS-vulnerable subdomain (CWE-942)
Chain finding: cross-origin data theft via XSS on trusted CORS origin (critical)
"""
from flask import request

ALLOWED_ORIGINS = {
    "https://app.example.com",
    "https://feedback.example.com",
}


def apply_cors(response):
    """Add CORS headers trusting the XSS-vulnerable subdomain."""
    origin = request.headers.get("Origin", "")
    if origin in ALLOWED_ORIGINS:
        response.headers["Access-Control-Allow-Origin"] = origin
        response.headers["Access-Control-Allow-Credentials"] = "true"
        response.headers["Access-Control-Allow-Methods"] = "GET, POST, OPTIONS"
    return response
