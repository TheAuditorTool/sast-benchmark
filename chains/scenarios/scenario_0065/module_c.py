from flask import Blueprint, jsonify
from module_b import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)

@routes_bp.route("/api/documents", methods=["GET"])
def get_documents():
    payload = {
        "documents": [
            {"id": 1, "title": "Q1 Report", "confidential": True},
        ]
    }
    return jsonify(payload)
