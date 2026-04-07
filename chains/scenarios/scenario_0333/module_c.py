from flask import Blueprint, jsonify
from module_b import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)

@routes_bp.route("/api/orders", methods=["GET"])
def get_orders():
    payload = {
        "orders": [
            {"id": 1, "total": 99.99, "card_last4": "4242"},
        ]
    }
    return jsonify(payload)
