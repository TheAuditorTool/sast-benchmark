"""Auth endpoints -- SAFE variant for md5_hmac_token.

POST /login issues an HMAC-SHA256-signed token.
GET  /protected verifies the token and grants access.

This file is IDENTICAL between vuln/ and safe/ variants (only
tokens.py changes).
"""
from flask import Blueprint, request, jsonify
from tokens import make_token, verify_token

auth_bp = Blueprint("auth", __name__)


@auth_bp.route("/login", methods=["POST"])
def login():
    """Issue a signed token for the supplied user_id."""
    user_id = request.json.get("user_id", "anonymous")
    token = make_token(user_id)
    return jsonify({"token": token})


# vuln-code-snippet start chain_md5_hmac_safe
@auth_bp.route("/protected")
def protected():
    """Return protected data if the HMAC-SHA256 token is valid."""
    token = request.headers.get("X-Token", "")
    user_id = verify_token(token)
    if not user_id:
        return jsonify({"error": "Forbidden"}), 403
    return jsonify({"data": "secret", "user": user_id})  # vuln-code-snippet safe-line chain_md5_hmac_safe
# vuln-code-snippet end chain_md5_hmac_safe
