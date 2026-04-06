"""CORS middleware -- VULNERABLE variant.

Sets Access-Control-Allow-Origin to the requesting origin but does NOT set
the Vary: Origin response header. Without Vary: Origin, a shared cache
(CDN or reverse proxy) may store one origin's response and serve it to a
different origin, effectively overriding the per-origin CORS policy.

Chain: no Vary: Origin -> CDN caches ACAO for origin A -> origin B gets cached ACAO of A -> CORS bypass
Individual findings: CORS response missing Vary: Origin (CWE-942)
Chain finding: CORS policy bypass via cache serving wrong origin's ACAO header (medium)
"""
from flask import request

ALLOWED_ORIGINS = {"https://app.example.com", "https://partner.example.com"}


def apply_cors(response):
    """Add CORS headers without the required Vary: Origin header."""
    origin = request.headers.get("Origin", "")
    if origin in ALLOWED_ORIGINS:
        response.headers["Access-Control-Allow-Origin"] = origin
        response.headers["Access-Control-Allow-Credentials"] = "true"
    return response
