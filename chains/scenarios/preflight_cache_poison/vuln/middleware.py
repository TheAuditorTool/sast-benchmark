"""CORS middleware -- VULNERABLE variant.

Responds to preflight OPTIONS requests with Access-Control-Max-Age set to
86400 seconds (24 hours). An attacker who can get a victim browser to issue
a preflight to a malicious endpoint first can poison the browser's preflight
cache, causing subsequent legitimate requests to that URL to use the cached
(attacker-influenced) permissions for the full 24-hour window.

Chain: extremely long preflight max-age -> poisoned cache persists 24h -> victim browser skips re-check
Individual findings: excessive CORS preflight max-age (CWE-942)
Chain finding: CORS policy bypass via long-lived preflight cache poisoning (medium)
"""
from flask import request


def apply_cors(response):
    """Add CORS headers with an excessively long preflight cache duration."""
    origin = request.headers.get("Origin", "")
    if origin:
        response.headers["Access-Control-Allow-Origin"] = origin
        response.headers["Access-Control-Allow-Credentials"] = "true"
        response.headers["Access-Control-Allow-Methods"] = "GET, POST, DELETE, OPTIONS"
        response.headers["Access-Control-Allow-Headers"] = "Content-Type, Authorization"
        response.headers["Access-Control-Max-Age"] = "86400"
    return response
