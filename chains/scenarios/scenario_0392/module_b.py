from flask import Blueprint, request, jsonify
import module_c

oauth_bp = Blueprint("oauth", __name__)

@oauth_bp.route("/token-exchange", methods=["POST"])
def token_exchange():
    code = request.json.get("code", "")
    token_request_body = {
        "grant_type": "authorization_code",
        "code": code,
        "client_id": config.OAUTH_CLIENT_ID,
        "client_secret": config.OAUTH_CLIENT_SECRET,
    }
    return jsonify({"token_url": config.OAUTH_TOKEN_URL, "body_keys": list(token_request_body.keys())})
