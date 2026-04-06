"""Sensitive data routes -- VULNERABLE variant.

GET /api/account returns account details including the session token.
The CORS middleware in middleware.py reflects any Origin with credentials
allowed, so a malicious site can trigger this endpoint from a victim's
browser and read the full response including the token.

Chain: any Origin reflected + credentials -> attacker's page reads /api/account -> token stolen
Individual findings: sensitive data exposed under open CORS with credentials (CWE-942)
Chain finding: cross-origin session token theft via reflected-origin policy (critical)
"""
from flask import Blueprint, jsonify, g
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)


@routes_bp.route("/api/account", methods=["GET"])
def get_account():
    """Return account details for the authenticated session."""
# vuln-code-snippet start chain_reflect_origin_vuln
    payload = {
        "user_id": 42,
        "email": "user@example.com",
        "session_token": "tok-secret-abc123",  # vuln-code-snippet vuln-line chain_reflect_origin_vuln
    }
# vuln-code-snippet end chain_reflect_origin_vuln
    return jsonify(payload)
