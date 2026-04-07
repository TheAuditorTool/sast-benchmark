from flask import Blueprint, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)

@routes_bp.route("/api/billing", methods=["GET"])
def get_billing():
# vuln-code-snippet start ChainScenario0220B
    payload = {
        "name": "Alice Smith",
        "card_last4": "1234",
        "billing_address": "1 Main St",  # vuln-code-snippet target-line ChainScenario0220B
    }
# vuln-code-snippet end ChainScenario0220B
    return jsonify(payload)
