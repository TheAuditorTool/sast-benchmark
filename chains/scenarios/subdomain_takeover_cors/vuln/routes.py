"""Sensitive data routes -- VULNERABLE variant.

GET /api/billing returns billing address and card details. The CORS
middleware trusts any *.example.com subdomain. If an attacker takes over
abandoned-staging.example.com, that origin passes the check and can read
this endpoint cross-origin with credentials.

Chain: taken-over subdomain origin trusted -> attacker reads billing data -> credit card info exfiltrated
Individual findings: billing data exposed under wildcard subdomain CORS (CWE-942)
Chain finding: financial data theft via subdomain-takeover CORS chain (critical)
"""
from flask import Blueprint, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)


@routes_bp.route("/api/billing", methods=["GET"])
def get_billing():
    """Return the current user's billing information."""
# vuln-code-snippet start chain_subdomain_cors_vuln
    payload = {
        "name": "Alice Smith",
        "card_last4": "1234",
        "billing_address": "1 Main St",  # vuln-code-snippet vuln-line chain_subdomain_cors_vuln
    }
# vuln-code-snippet end chain_subdomain_cors_vuln
    return jsonify(payload)
