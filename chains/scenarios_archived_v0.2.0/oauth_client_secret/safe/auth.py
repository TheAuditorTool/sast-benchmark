"""OAuth token exchange handler for oauth_client_secret scenario -- SAFE variant.

POST /token-exchange performs the OAuth flow using a client secret
loaded from an environment variable. No secret is embedded in source.

Chain broken: config.OAUTH_CLIENT_SECRET from env -> no hardcoded value -> no impersonation
"""
from flask import Blueprint, request, jsonify
import config

oauth_bp = Blueprint("oauth", __name__)


# vuln-code-snippet start chain_oauth_secret_safe
@oauth_bp.route("/token-exchange", methods=["POST"])
def token_exchange():
    """Exchange an authorization code using an environment-sourced client secret."""
    code = request.json.get("code", "")
    token_request_body = {
        "grant_type": "authorization_code",
        "code": code,
        "client_id": config.OAUTH_CLIENT_ID,
        "client_secret": config.OAUTH_CLIENT_SECRET,  # vuln-code-snippet safe-line chain_oauth_secret_safe
    }
    return jsonify({"token_url": config.OAUTH_TOKEN_URL, "body_keys": list(token_request_body.keys())})
# vuln-code-snippet end chain_oauth_secret_safe
