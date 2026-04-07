from flask import Blueprint, jsonify, g
from module_b import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)

@routes_bp.route("/api/account", methods=["GET"])
def get_account():
    payload = {
        "user_id": 42,
        "email": "user@example.com",
        "session_token": "tok-secret-abc123",
    }
    return jsonify(payload)
