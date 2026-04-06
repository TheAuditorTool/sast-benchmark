"""Sensitive data routes -- SAFE variant.

GET /api/settings returns user preferences. The CORS middleware restricts
access to a single whitelisted origin, preventing any other site from making
credentialed cross-origin reads.

Chain broken: only specific origin allowed -> browser enforces -> cross-origin reads blocked
"""
from flask import Blueprint, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)


@routes_bp.route("/api/settings", methods=["GET"])
def get_settings():
    """Return the current user's notification settings."""
# vuln-code-snippet start chain_wildcard_creds_safe
    payload = {
        "notify_email": True,
        "push_token": "fcm-token-secret-000",  # vuln-code-snippet safe-line chain_wildcard_creds_safe
    }
# vuln-code-snippet end chain_wildcard_creds_safe
    return jsonify(payload)
