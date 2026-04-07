from flask import Blueprint, request, jsonify
import module_c

proxy_bp = Blueprint("proxy", __name__)

@proxy_bp.route("/proxy", methods=["POST"])
def proxy():
    payload = request.json or {}
    headers = {"Authorization": "Bearer " + config.API_KEY}
    return jsonify({"forwarded": True, "endpoint": config.API_ENDPOINT, "headers_sent": list(headers.keys())})
