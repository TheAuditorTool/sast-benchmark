"""CORS middleware -- VULNERABLE variant.

Trusts any subdomain of example.com by checking if the origin ends with
'.example.com'. If an attacker takes over an unused subdomain (e.g.
abandoned-staging.example.com points to an attacker-controlled server),
that origin passes the check and gains full credentialed CORS access.

Chain: wildcard subdomain CORS + subdomain takeover -> attacker origin trusted -> data stolen
Individual findings: CORS trusts all subdomains (CWE-942)
Chain finding: cross-origin data theft via subdomain takeover CORS bypass (critical)
"""
from flask import request


def apply_cors(response):
    """Add CORS headers for any subdomain of example.com."""
    origin = request.headers.get("Origin", "")
    if origin.endswith(".example.com") or origin == "https://example.com":
        response.headers["Access-Control-Allow-Origin"] = origin
        response.headers["Access-Control-Allow-Credentials"] = "true"
        response.headers["Access-Control-Allow-Methods"] = "GET, POST, OPTIONS"
    return response
