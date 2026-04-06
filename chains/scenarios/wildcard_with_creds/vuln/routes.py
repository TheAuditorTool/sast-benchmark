"""Sensitive data routes -- VULNERABLE variant.

GET /api/settings returns user preferences including notification tokens.
The CORS policy uses wildcard with credentials, which is spec-prohibited
in browsers but signals a misconfigured policy; non-browser HTTP clients
and certain environments treat this combination as fully permissive.

Chain: wildcard + credentials -> any origin reads settings -> notification token leaked
Individual findings: settings exposed under wildcard CORS + credentials (CWE-942)
Chain finding: token exfiltration via wildcard CORS misconfiguration (high)
"""
from flask import Blueprint, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)


@routes_bp.route("/api/settings", methods=["GET"])
def get_settings():
    """Return the current user's notification settings."""
# vuln-code-snippet start chain_wildcard_creds_vuln
    payload = {
        "notify_email": True,
        "push_token": "fcm-token-secret-000",  # vuln-code-snippet vuln-line chain_wildcard_creds_vuln
    }
# vuln-code-snippet end chain_wildcard_creds_vuln
    return jsonify(payload)
