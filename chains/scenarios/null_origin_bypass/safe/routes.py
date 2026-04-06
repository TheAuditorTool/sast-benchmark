"""Sensitive data routes -- SAFE variant.

GET /api/profile returns user profile data. The CORS middleware no longer
accepts Origin: null, so sandboxed iframe attacks are blocked by the browser.

Chain broken: null origin not in allowlist -> CORS header not sent -> browser blocks iframe read
"""
from flask import Blueprint, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)


@routes_bp.route("/api/profile", methods=["GET"])
def get_profile():
    """Return the current user's profile including API key."""
# vuln-code-snippet start chain_null_origin_safe
    payload = {
        "username": "alice",
        "api_key": "sk-live-xyz789",  # vuln-code-snippet safe-line chain_null_origin_safe
    }
# vuln-code-snippet end chain_null_origin_safe
    return jsonify(payload)
