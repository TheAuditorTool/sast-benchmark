"""Auth endpoints for xor_cipher_token scenario.

POST /token issues an encrypted token.
GET  /protected decrypts the token and returns protected data.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
import base64
from flask import Blueprint, request, jsonify
from crypto import encrypt, decrypt

auth_bp = Blueprint("auth", __name__)


@auth_bp.route("/token", methods=["POST"])
def issue():
    """Issue an encrypted token for the supplied user:role payload."""
    payload = (request.json or {}).get("payload", "user:guest")
    ct = encrypt(payload)
    return jsonify({"token": base64.b64encode(ct).decode()})


# vuln-code-snippet start chain_xor_cipher_vuln
@auth_bp.route("/protected")
def protected():
    """Decrypt token and grant admin access if role is admin."""
    raw = request.headers.get("X-Token", "")
    try:
        ct = base64.b64decode(raw)
        payload = decrypt(ct)
    except Exception:
        return jsonify({"error": "Bad token"}), 400
    if ":admin" not in payload:
        return jsonify({"error": "Forbidden"}), 403
    return jsonify({"secret": "admin data"})  # vuln-code-snippet vuln-line chain_xor_cipher_vuln
# vuln-code-snippet end chain_xor_cipher_vuln
