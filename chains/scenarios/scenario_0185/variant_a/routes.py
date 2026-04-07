from flask import Blueprint, jsonify, g
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)

@routes_bp.route("/api/account", methods=["GET"])
def get_account():
# vuln-code-snippet start ChainScenario0185A
    payload = {
        "user_id": 42,
        "email": "user@example.com",
        "session_token": "tok-secret-abc123",  # vuln-code-snippet target-line ChainScenario0185A
    }
# vuln-code-snippet end ChainScenario0185A
    return jsonify(payload)
