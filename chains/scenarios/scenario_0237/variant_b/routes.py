from flask import Blueprint, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)

@routes_bp.route("/api/orders", methods=["GET"])
def get_orders():
# vuln-code-snippet start ChainScenario0237B
    payload = {
        "orders": [
            {"id": 1, "total": 99.99, "card_last4": "4242"},  # vuln-code-snippet target-line ChainScenario0237B
        ]
    }
# vuln-code-snippet end ChainScenario0237B
    return jsonify(payload)
