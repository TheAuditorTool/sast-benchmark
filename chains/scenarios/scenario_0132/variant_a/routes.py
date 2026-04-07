from flask import Blueprint, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)

@routes_bp.route("/api/settings", methods=["GET"])
def get_settings():
# vuln-code-snippet start ChainScenario0132A
    payload = {
        "notify_email": True,
        "push_token": "fcm-token-secret-000",  # vuln-code-snippet target-line ChainScenario0132A
    }
# vuln-code-snippet end ChainScenario0132A
    return jsonify(payload)
