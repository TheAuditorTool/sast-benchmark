from flask import Blueprint, request, jsonify
from module_b import check_ws_origin

routes_bp = Blueprint("routes", __name__)

@routes_bp.route("/api/ws-token", methods=["GET"])
def get_ws_token():
    if not check_ws_origin():
        return jsonify({"error": "origin not allowed"}), 403
    token = "ws-tok-insecure-abc"
    return jsonify({"ws_token": token})
