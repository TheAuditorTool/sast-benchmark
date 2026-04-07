"""Auth endpoints for short_secret_jwt scenario.

POST /login issues a JWT.
GET  /protected verifies the JWT and returns protected data.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Blueprint, request, jsonify
from tokens import make_token, verify_token

auth_bp = Blueprint("auth", __name__)


@auth_bp.route("/login", methods=["POST"])
def login():
    """Issue a JWT for the supplied user_id."""
    user_id = request.json.get("user_id", "anonymous")
    token = make_token({"sub": user_id, "role": "user"})
    return jsonify({"token": token})


# vuln-code-snippet start chain_short_secret_jwt_vuln
@auth_bp.route("/protected")
def protected():
    """Return protected data if the JWT is valid."""
    token = request.headers.get("Authorization", "").removeprefix("Bearer ")
    claims = verify_token(token)
    if not claims:
        return jsonify({"error": "Forbidden"}), 403
    return jsonify({"data": "secret", "claims": claims})  # vuln-code-snippet vuln-line chain_short_secret_jwt_vuln
# vuln-code-snippet end chain_short_secret_jwt_vuln
