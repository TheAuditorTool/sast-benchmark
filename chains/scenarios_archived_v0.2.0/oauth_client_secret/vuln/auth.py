"""OAuth token exchange handler for oauth_client_secret scenario -- VULNERABLE variant.

POST /token-exchange performs the OAuth client-credentials flow using
the hardcoded client secret. An attacker who reads the source can
exchange tokens as this application.

Chain: POST /token-exchange -> config.OAUTH_CLIENT_SECRET -> token request -> impersonation
"""
from flask import Blueprint, request, jsonify
import config

oauth_bp = Blueprint("oauth", __name__)


# vuln-code-snippet start chain_oauth_secret_vuln
@oauth_bp.route("/token-exchange", methods=["POST"])
def token_exchange():
    """Exchange an authorization code for an access token using the hardcoded client secret."""
    code = request.json.get("code", "")
    token_request_body = {
        "grant_type": "authorization_code",
        "code": code,
        "client_id": config.OAUTH_CLIENT_ID,
        "client_secret": config.OAUTH_CLIENT_SECRET,  # vuln-code-snippet vuln-line chain_oauth_secret_vuln
    }
    return jsonify({"token_url": config.OAUTH_TOKEN_URL, "body_keys": list(token_request_body.keys())})
# vuln-code-snippet end chain_oauth_secret_vuln
