from flask import Blueprint, request, jsonify
import config

oauth_bp = Blueprint("oauth", __name__)

# vuln-code-snippet start ChainScenario0124A
@oauth_bp.route("/token-exchange", methods=["POST"])
def token_exchange():
    code = request.json.get("code", "")
    token_request_body = {
        "grant_type": "authorization_code",
        "code": code,
        "client_id": config.OAUTH_CLIENT_ID,
        "client_secret": config.OAUTH_CLIENT_SECRET,  # vuln-code-snippet target-line ChainScenario0124A
    }
    return jsonify({"token_url": config.OAUTH_TOKEN_URL, "body_keys": list(token_request_body.keys())})
# vuln-code-snippet end ChainScenario0124A
