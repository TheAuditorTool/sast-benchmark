"""Sensitive data routes -- VULNERABLE variant.

GET /api/orders returns the user's order history with payment info.
The CORS middleware uses an unanchored regex, so an attacker operating
evil-example.com or example.com.evil.io passes the check and can read
credentialed responses from a victim's browser.

Chain: regex bypass -> CORS granted to attacker domain -> order and payment data stolen
Individual findings: order data exposed under bypassed CORS policy (CWE-942)
Chain finding: payment data theft via regex-bypass CORS (high)
"""
from flask import Blueprint, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)


@routes_bp.route("/api/orders", methods=["GET"])
def get_orders():
    """Return the current user's order history."""
# vuln-code-snippet start chain_regex_origin_vuln
    payload = {
        "orders": [
            {"id": 1, "total": 99.99, "card_last4": "4242"},  # vuln-code-snippet vuln-line chain_regex_origin_vuln
        ]
    }
# vuln-code-snippet end chain_regex_origin_vuln
    return jsonify(payload)
