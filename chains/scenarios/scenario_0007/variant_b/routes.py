from flask import Blueprint, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)

@routes_bp.route("/api/documents", methods=["GET"])
def get_documents():
# vuln-code-snippet start ChainScenario0007B
    payload = {
        "documents": [
            {"id": 1, "title": "Q1 Report", "confidential": True},  # vuln-code-snippet target-line ChainScenario0007B
        ]
    }
# vuln-code-snippet end ChainScenario0007B
    return jsonify(payload)
