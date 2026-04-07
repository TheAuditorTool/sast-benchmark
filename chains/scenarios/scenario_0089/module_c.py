from flask import Blueprint, jsonify
from module_b import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)

@routes_bp.route("/api/billing", methods=["GET"])
def get_billing():
    payload = {
        "name": "Alice Smith",
        "card_last4": "1234",
        "billing_address": "1 Main St",
    }
    return jsonify(payload)
