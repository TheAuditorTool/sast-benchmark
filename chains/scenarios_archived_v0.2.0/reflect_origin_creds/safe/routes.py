"""Sensitive data routes -- SAFE variant.

GET /api/account returns account details. The CORS middleware in middleware.py
only grants access to whitelisted origins. An attacker's origin will not
receive the CORS header, so the browser will block the cross-origin read.

Chain broken: unknown origins get no CORS header -> browser blocks cross-origin reads -> data protected
"""
from flask import Blueprint, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)


@routes_bp.route("/api/account", methods=["GET"])
def get_account():
    """Return account details for the authenticated session."""
# vuln-code-snippet start chain_reflect_origin_safe
    payload = {
        "user_id": 42,
        "email": "user@example.com",
        "session_token": "tok-secret-abc123",  # vuln-code-snippet safe-line chain_reflect_origin_safe
    }
# vuln-code-snippet end chain_reflect_origin_safe
    return jsonify(payload)
