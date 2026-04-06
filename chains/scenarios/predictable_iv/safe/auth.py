"""Auth endpoints for predictable_iv scenario -- SAFE variant.

POST /encrypt returns the ciphertext of the supplied payload (random IV).
GET  /check verifies a submitted ciphertext.

This file is IDENTICAL between vuln/ and safe/ variants (only
crypto.py changes).
"""
import base64
from flask import Blueprint, request, jsonify
from crypto import encrypt

auth_bp = Blueprint("auth", __name__)

_KNOWN_PAYLOAD = "role=admin"


@auth_bp.route("/encrypt", methods=["POST"])
def do_encrypt():
    """Encrypt the supplied payload with a random IV."""
    payload = request.json.get("payload", "role=user")
    ct = encrypt(payload)
    return jsonify({"ciphertext": base64.b64encode(ct).decode()})


# vuln-code-snippet start chain_predictable_iv_safe
@auth_bp.route("/check")
def check():
    """Accept ciphertext and compare to admin token (random IV)."""
    raw = request.headers.get("X-Cipher", "")
    try:
        ct = base64.b64decode(raw)
    except Exception:
        return jsonify({"error": "Bad input"}), 400
    expected = encrypt(_KNOWN_PAYLOAD)
    match = ct == expected
    return jsonify({"match": match})  # vuln-code-snippet safe-line chain_predictable_iv_safe
# vuln-code-snippet end chain_predictable_iv_safe
