from flask import Blueprint, request, jsonify
import config

proxy_bp = Blueprint("proxy", __name__)

# vuln-code-snippet start ChainScenario0009B
@proxy_bp.route("/proxy", methods=["POST"])
def proxy():
    payload = request.json or {}
    headers = {"Authorization": "Bearer " + config.API_KEY}  # vuln-code-snippet target-line ChainScenario0009B
    return jsonify({"forwarded": True, "endpoint": config.API_ENDPOINT, "headers_sent": list(headers.keys())})
# vuln-code-snippet end ChainScenario0009B
