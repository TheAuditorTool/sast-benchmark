"""Sensitive data routes -- SAFE variant.

GET /api/private-notes returns private notes. The CORS middleware no longer
trusts feedback.example.com. An XSS payload on that subdomain cannot make
credentialed cross-origin requests to this endpoint.

Chain broken: vulnerable subdomain removed from allowlist -> XSS script at feedback cannot read API -> data protected
"""
from flask import Blueprint, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)


@routes_bp.route("/api/private-notes", methods=["GET"])
def get_notes():
    """Return the current user's private notes."""
# vuln-code-snippet start chain_cors_xss_safe
    payload = {
        "notes": [
            {"id": 1, "body": "meeting at 3pm - confidential details"},  # vuln-code-snippet safe-line chain_cors_xss_safe
        ]
    }
# vuln-code-snippet end chain_cors_xss_safe
    return jsonify(payload)
