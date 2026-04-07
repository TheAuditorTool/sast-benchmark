from flask import Blueprint, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)

@routes_bp.route("/internal/metrics", methods=["GET"])
def get_metrics():
# vuln-code-snippet start ChainScenario0192A
    payload = {
        "db_connections": 42,
        "queue_depth": 7,
        "internal_ip": "10.0.1.50",  # vuln-code-snippet target-line ChainScenario0192A
    }
# vuln-code-snippet end ChainScenario0192A
    return jsonify(payload)
