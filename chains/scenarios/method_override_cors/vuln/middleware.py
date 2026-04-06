"""CORS + method-override middleware -- VULNERABLE variant.

The CORS policy allows only GET (a "simple" method that skips preflight),
but the server also honors an X-HTTP-Method-Override header. An attacker
can send a credentialed GET with X-HTTP-Method-Override: DELETE from any
origin, bypassing the CORS preflight restriction and triggering a DELETE.

Chain: GET allowed without preflight + method override -> DELETE executed cross-origin without preflight
Individual findings: CORS simple method + server-side method override (CWE-942)
Chain finding: CORS preflight bypass via GET + X-HTTP-Method-Override (high)
"""
from flask import request


def get_effective_method() -> str:
    """Return the effective HTTP method, honoring X-HTTP-Method-Override."""
    override = request.headers.get("X-HTTP-Method-Override", "")
    if override:
        return override.upper()
    return request.method


def apply_cors(response):
    """Add CORS headers allowing only GET (skips preflight)."""
    origin = request.headers.get("Origin", "")
    if origin:
        response.headers["Access-Control-Allow-Origin"] = origin
        response.headers["Access-Control-Allow-Credentials"] = "true"
        response.headers["Access-Control-Allow-Methods"] = "GET"
    return response
