from flask import Blueprint, jsonify
from module_b import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)

@routes_bp.route("/api/private-notes", methods=["GET"])
def get_notes():
    payload = {
        "notes": [
            {"id": 1, "body": "meeting at 3pm - confidential details"},
        ]
    }
    return jsonify(payload)
