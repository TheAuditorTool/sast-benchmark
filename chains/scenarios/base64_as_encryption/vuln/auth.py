"""Auth endpoints for base64_as_encryption scenario.

POST /token issues a token for the supplied claims.
GET  /protected verifies the token and returns protected data.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Blueprint, request, jsonify
from crypto import encrypt_claims, decrypt_claims

auth_bp = Blueprint("auth", __name__)


@auth_bp.route("/token", methods=["POST"])
def issue():
    """Issue a token encoding the supplied claims."""
    claims = request.json or {"role": "user"}
    return jsonify({"token": encrypt_claims(claims)})


# vuln-code-snippet start chain_base64_encrypt_vuln
@auth_bp.route("/protected")
def protected():
    """Return admin data if the token contains role=admin."""
    token = request.headers.get("X-Token", "")
    claims = decrypt_claims(token)
    if not claims:
        return jsonify({"error": "Forbidden"}), 403
    if claims.get("role") != "admin":
        return jsonify({"error": "Forbidden"}), 403
    return jsonify({"secret": "admin data"})  # vuln-code-snippet vuln-line chain_base64_encrypt_vuln
# vuln-code-snippet end chain_base64_encrypt_vuln
