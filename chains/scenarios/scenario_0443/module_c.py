from flask import Blueprint, jsonify
from module_b import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)

@routes_bp.route("/api/profile", methods=["GET"])
def get_profile():
    payload = {
        "username": "alice",
        "api_key": "sk-live-xyz789",
    }
    return jsonify(payload)
