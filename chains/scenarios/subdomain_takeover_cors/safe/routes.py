"""Sensitive data routes -- SAFE variant.

GET /api/billing returns billing information. The CORS middleware uses an
explicit allowlist so only known origins are trusted. Subdomain takeover
attacks cannot pass this check.

Chain broken: only explicit origins allowed -> taken-over subdomain rejected -> data protected
"""
from flask import Blueprint, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)


@routes_bp.route("/api/billing", methods=["GET"])
def get_billing():
    """Return the current user's billing information."""
# vuln-code-snippet start chain_subdomain_cors_safe
    payload = {
        "name": "Alice Smith",
        "card_last4": "1234",
        "billing_address": "1 Main St",  # vuln-code-snippet safe-line chain_subdomain_cors_safe
    }
# vuln-code-snippet end chain_subdomain_cors_safe
    return jsonify(payload)
