from flask import Blueprint, jsonify
from module_b import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)

@routes_bp.route("/api/settings", methods=["GET"])
def get_settings():
    payload = {
        "notify_email": True,
        "push_token": "fcm-token-secret-000",
    }
    return jsonify(payload)
