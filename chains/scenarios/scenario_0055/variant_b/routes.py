from flask import Blueprint, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)

@routes_bp.route("/api/private-notes", methods=["GET"])
def get_notes():
# vuln-code-snippet start ChainScenario0055B
    payload = {
        "notes": [
            {"id": 1, "body": "meeting at 3pm - confidential details"},  # vuln-code-snippet target-line ChainScenario0055B
        ]
    }
# vuln-code-snippet end ChainScenario0055B
    return jsonify(payload)
