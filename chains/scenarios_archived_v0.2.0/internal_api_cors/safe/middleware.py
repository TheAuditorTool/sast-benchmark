"""CORS middleware -- SAFE variant.

The internal API does not set any CORS headers. Without an
Access-Control-Allow-Origin header, browsers apply the same-origin policy
and block all cross-origin reads. Only same-origin or server-side callers
can access this endpoint.

Chain broken: no CORS headers emitted -> browser blocks cross-origin reads -> internal data protected
"""
from flask import request


def apply_cors(response):
    """Do not add CORS headers for internal API endpoints."""
    return response
