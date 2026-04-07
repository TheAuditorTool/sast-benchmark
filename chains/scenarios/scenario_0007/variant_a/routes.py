from flask import Blueprint, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)

@routes_bp.route("/api/documents", methods=["GET"])
def get_documents():
# vuln-code-snippet start ChainScenario0007A
    payload = {
        "documents": [
            {"id": 1, "title": "Q1 Report", "confidential": True},  # vuln-code-snippet target-line ChainScenario0007A
        ]
    }
# vuln-code-snippet end ChainScenario0007A
    return jsonify(payload)
