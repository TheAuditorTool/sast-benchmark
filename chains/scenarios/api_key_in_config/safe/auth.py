"""API proxy authentication for api_key_in_config scenario -- SAFE variant.

POST /proxy forwards requests using an API key sourced from an
environment variable. No key is embedded in source.

Chain broken: config.API_KEY from env -> no hardcoded key -> no unauthorized access
"""
from flask import Blueprint, request, jsonify
import config

proxy_bp = Blueprint("proxy", __name__)


# vuln-code-snippet start chain_api_key_config_safe
@proxy_bp.route("/proxy", methods=["POST"])
def proxy():
    """Forward a request to the external API using an environment-sourced API key."""
    payload = request.json or {}
    headers = {"Authorization": "Bearer " + config.API_KEY}  # vuln-code-snippet safe-line chain_api_key_config_safe
    return jsonify({"forwarded": True, "endpoint": config.API_ENDPOINT, "headers_sent": list(headers.keys())})
# vuln-code-snippet end chain_api_key_config_safe
