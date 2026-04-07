"""Sensitive data routes -- VULNERABLE variant.

GET /api/profile returns user profile data including the API key.
The CORS middleware accepts Origin: null with credentials, so an attacker
embedding a sandboxed iframe can read this endpoint from any web page.

Chain: null origin allowed -> sandboxed iframe sends credentialed request -> profile + API key stolen
Individual findings: sensitive data exposed under null-origin CORS policy (CWE-942)
Chain finding: credential theft via null-origin CORS bypass (high)
"""
from flask import Blueprint, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)


@routes_bp.route("/api/profile", methods=["GET"])
def get_profile():
    """Return the current user's profile including API key."""
# vuln-code-snippet start chain_null_origin_vuln
    payload = {
        "username": "alice",
        "api_key": "sk-live-xyz789",  # vuln-code-snippet vuln-line chain_null_origin_vuln
    }
# vuln-code-snippet end chain_null_origin_vuln
    return jsonify(payload)
