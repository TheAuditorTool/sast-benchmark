from flask import Blueprint, jsonify
from module_b import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)

@routes_bp.route("/internal/metrics", methods=["GET"])
def get_metrics():
    payload = {
        "db_connections": 42,
        "queue_depth": 7,
        "internal_ip": "10.0.1.50",
    }
    return jsonify(payload)
