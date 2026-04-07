"""Auth endpoints for hardcoded_jwt_secret scenario -- SAFE variant.

POST /login issues a JWT signed with an env-sourced secret.
GET  /protected verifies the JWT.

This file is IDENTICAL between vuln/ and safe/ variants (only
tokens.py changes).
"""
from flask import Blueprint, request, jsonify
from tokens import make_token, verify_token

auth_bp = Blueprint("auth", __name__)


@auth_bp.route("/login", methods=["POST"])
def login():
    """Issue a JWT for the supplied user."""
    user = (request.json or {}).get("user", "guest")
    return jsonify({"token": make_token({"sub": user, "role": "user"})})


# vuln-code-snippet start chain_hardcoded_jwt_safe
@auth_bp.route("/protected")
def protected():
    """Return protected data if the JWT (env-secret) is valid."""
    token = request.headers.get("Authorization", "").removeprefix("Bearer ")
    claims = verify_token(token)
    if not claims:
        return jsonify({"error": "Forbidden"}), 403
    return jsonify({"data": "protected", "claims": claims})  # vuln-code-snippet safe-line chain_hardcoded_jwt_safe
# vuln-code-snippet end chain_hardcoded_jwt_safe
