"""Auth endpoints for ecb_mode_token scenario -- SAFE variant.

POST /token issues an AES-CBC encrypted token.
GET  /verify accepts and decrypts a token.

This file is IDENTICAL between vuln/ and safe/ variants (only
crypto.py changes).
"""
import base64
from flask import Blueprint, request, jsonify
from crypto import encrypt_token, decrypt_token

auth_bp = Blueprint("auth", __name__)


@auth_bp.route("/token", methods=["POST"])
def issue_token():
    """Issue an AES-CBC encrypted token for the supplied payload."""
    payload = request.json.get("payload", "user:guest:role:user")
    ciphertext = encrypt_token(payload)
    return jsonify({"token": base64.b64encode(ciphertext).decode()})


# vuln-code-snippet start chain_ecb_mode_safe
@auth_bp.route("/verify")
def verify():
    """Decrypt and return the token payload (CBC-encrypted)."""
    raw = request.headers.get("X-Token", "")
    try:
        ciphertext = base64.b64decode(raw)
    except Exception:
        return jsonify({"error": "Bad token"}), 400
    payload = decrypt_token(ciphertext)
    return jsonify({"payload": payload})  # vuln-code-snippet safe-line chain_ecb_mode_safe
# vuln-code-snippet end chain_ecb_mode_safe
