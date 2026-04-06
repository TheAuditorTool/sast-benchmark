"""API proxy authentication for api_key_in_config scenario -- VULNERABLE variant.

POST /proxy forwards requests to the external API using the hardcoded
API key from config. The key is placed in the Authorization header,
so any call succeeds without the caller needing to supply credentials.

Chain: POST /proxy -> config.API_KEY in header -> external API call
"""
from flask import Blueprint, request, jsonify
import config

proxy_bp = Blueprint("proxy", __name__)


# vuln-code-snippet start chain_api_key_config_vuln
@proxy_bp.route("/proxy", methods=["POST"])
def proxy():
    """Forward a request to the external API using the configured API key."""
    payload = request.json or {}
    headers = {"Authorization": "Bearer " + config.API_KEY}  # vuln-code-snippet vuln-line chain_api_key_config_vuln
    return jsonify({"forwarded": True, "endpoint": config.API_ENDPOINT, "headers_sent": list(headers.keys())})
# vuln-code-snippet end chain_api_key_config_vuln
