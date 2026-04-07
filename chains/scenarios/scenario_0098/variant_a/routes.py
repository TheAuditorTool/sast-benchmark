from flask import Blueprint, request, jsonify
from middleware import check_ws_origin

routes_bp = Blueprint("routes", __name__)

@routes_bp.route("/api/ws-token", methods=["GET"])
def get_ws_token():
# vuln-code-snippet start ChainScenario0098A
    if not check_ws_origin():
        return jsonify({"error": "origin not allowed"}), 403
    token = "ws-tok-insecure-abc"  # vuln-code-snippet target-line ChainScenario0098A
# vuln-code-snippet end ChainScenario0098A
    return jsonify({"ws_token": token})
