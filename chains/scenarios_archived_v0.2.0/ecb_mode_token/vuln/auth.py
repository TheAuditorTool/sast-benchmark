"""Auth endpoints for ecb_mode_token scenario.

POST /token issues an encrypted token.
GET  /verify accepts and decrypts a token.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
import base64
from flask import Blueprint, request, jsonify
from crypto import encrypt_token, decrypt_token

auth_bp = Blueprint("auth", __name__)


@auth_bp.route("/token", methods=["POST"])
def issue_token():
    """Issue an encrypted token for the supplied payload."""
    payload = request.json.get("payload", "user:guest:role:user")
    ciphertext = encrypt_token(payload)
    return jsonify({"token": base64.b64encode(ciphertext).decode()})


# vuln-code-snippet start chain_ecb_mode_vuln
@auth_bp.route("/verify")
def verify():
    """Decrypt and return the token payload."""
    raw = request.headers.get("X-Token", "")
    try:
        ciphertext = base64.b64decode(raw)
    except Exception:
        return jsonify({"error": "Bad token"}), 400
    payload = decrypt_token(ciphertext)
    return jsonify({"payload": payload})  # vuln-code-snippet vuln-line chain_ecb_mode_vuln
# vuln-code-snippet end chain_ecb_mode_vuln
