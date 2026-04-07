"""Auth endpoints for base64_as_encryption scenario -- SAFE variant.

POST /token issues an HMAC-signed token.
GET  /protected verifies the token signature and returns protected data.

This file is IDENTICAL between vuln/ and safe/ variants (only
crypto.py changes).
"""
from flask import Blueprint, request, jsonify
from crypto import encrypt_claims, decrypt_claims

auth_bp = Blueprint("auth", __name__)


@auth_bp.route("/token", methods=["POST"])
def issue():
    """Issue an HMAC-signed token encoding the supplied claims."""
    claims = request.json or {"role": "user"}
    return jsonify({"token": encrypt_claims(claims)})


# vuln-code-snippet start chain_base64_encrypt_safe
@auth_bp.route("/protected")
def protected():
    """Return admin data if the signed token contains role=admin."""
    token = request.headers.get("X-Token", "")
    claims = decrypt_claims(token)
    if not claims:
        return jsonify({"error": "Forbidden"}), 403
    if claims.get("role") != "admin":
        return jsonify({"error": "Forbidden"}), 403
    return jsonify({"secret": "admin data"})  # vuln-code-snippet safe-line chain_base64_encrypt_safe
# vuln-code-snippet end chain_base64_encrypt_safe
