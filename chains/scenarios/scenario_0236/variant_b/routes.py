from flask import Blueprint, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)

@routes_bp.route("/api/profile", methods=["GET"])
def get_profile():
# vuln-code-snippet start ChainScenario0236B
    payload = {
        "username": "alice",
        "api_key": "sk-live-xyz789",  # vuln-code-snippet target-line ChainScenario0236B
    }
# vuln-code-snippet end ChainScenario0236B
    return jsonify(payload)
