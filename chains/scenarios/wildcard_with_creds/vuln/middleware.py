"""CORS middleware -- VULNERABLE variant.

Sets Access-Control-Allow-Origin: * alongside Access-Control-Allow-Credentials:
true. Although modern browsers reject this combination, some HTTP clients and
older implementations do not, and the intent exposes all authenticated
endpoints to any origin. The combination signals a misconfigured policy.

Chain: wildcard ACAO + credentials header present -> non-browser clients bypass browser restriction -> data read
Individual findings: CORS wildcard with credentials flag (CWE-942)
Chain finding: cross-origin data exposure via wildcard CORS with credentials (high)
"""
from flask import request


def apply_cors(response):
    """Add wildcard CORS headers with credentials flag."""
    response.headers["Access-Control-Allow-Origin"] = "*"
    response.headers["Access-Control-Allow-Credentials"] = "true"
    response.headers["Access-Control-Allow-Methods"] = "GET, POST, OPTIONS"
    response.headers["Access-Control-Allow-Headers"] = "Content-Type, Authorization"
    return response
