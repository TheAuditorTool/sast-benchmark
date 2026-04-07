"""CORS middleware -- VULNERABLE variant.

The GraphQL endpoint has an open CORS policy that reflects any origin with
credentials allowed. GraphQL introspection and data queries are thus readable
from any website, exposing the full schema and all authenticated data to
cross-origin attackers.

Chain: open CORS on GraphQL -> any origin runs introspection + queries -> schema and data stolen
Individual findings: GraphQL endpoint open CORS with credentials (CWE-942)
Chain finding: GraphQL schema and data exfiltration via open CORS (critical)
"""
from flask import request


def apply_cors(response):
    """Add CORS headers reflecting any origin for the GraphQL endpoint."""
    origin = request.headers.get("Origin", "")
    if origin:
        response.headers["Access-Control-Allow-Origin"] = origin
        response.headers["Access-Control-Allow-Credentials"] = "true"
        response.headers["Access-Control-Allow-Methods"] = "GET, POST, OPTIONS"
        response.headers["Access-Control-Allow-Headers"] = "Content-Type, Authorization"
    return response
