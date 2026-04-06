"""Sensitive data routes -- SAFE variant.

GET /api/orders returns order history. The CORS middleware uses a fully
anchored regex, so only the exact legitimate origin matches. Attacker-
controlled domains are rejected and the browser blocks their cross-origin reads.

Chain broken: anchored regex rejects evil domains -> CORS not granted -> browser blocks read
"""
from flask import Blueprint, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)


@routes_bp.route("/api/orders", methods=["GET"])
def get_orders():
    """Return the current user's order history."""
# vuln-code-snippet start chain_regex_origin_safe
    payload = {
        "orders": [
            {"id": 1, "total": 99.99, "card_last4": "4242"},  # vuln-code-snippet safe-line chain_regex_origin_safe
        ]
    }
# vuln-code-snippet end chain_regex_origin_safe
    return jsonify(payload)
