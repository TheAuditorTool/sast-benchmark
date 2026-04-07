"""CORS middleware -- VULNERABLE variant.

An internal API intended for server-to-server calls has been deployed with
an open CORS policy (wildcard origin, no credentials). Although the intent
was server-side use only, the open CORS header allows browser-based clients
from any origin to call this API and read the response.

Chain: internal API exposed via open CORS -> external browser reads internal data -> data theft
Individual findings: internal API with open CORS policy (CWE-942)
Chain finding: internal data exposure via unintended CORS on internal API (high)
"""
from flask import request


def apply_cors(response):
    """Add permissive CORS headers intended for server-side calls only."""
    response.headers["Access-Control-Allow-Origin"] = "*"
    response.headers["Access-Control-Allow-Methods"] = "GET, POST, OPTIONS"
    response.headers["Access-Control-Allow-Headers"] = "Content-Type"
    return response
