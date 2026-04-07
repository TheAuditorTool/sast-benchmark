"""Auth endpoints for predictable_iv scenario.

POST /encrypt returns the ciphertext of the supplied payload.
GET  /check verifies a submitted ciphertext matches a known value.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
import base64
from flask import Blueprint, request, jsonify
from crypto import encrypt

auth_bp = Blueprint("auth", __name__)

_KNOWN_PAYLOAD = "role=admin"


@auth_bp.route("/encrypt", methods=["POST"])
def do_encrypt():
    """Encrypt the supplied payload and return base64 ciphertext."""
    payload = request.json.get("payload", "role=user")
    ct = encrypt(payload)
    return jsonify({"ciphertext": base64.b64encode(ct).decode()})


# vuln-code-snippet start chain_predictable_iv_vuln
@auth_bp.route("/check")
def check():
    """Accept ciphertext and compare to admin token."""
    raw = request.headers.get("X-Cipher", "")
    try:
        ct = base64.b64decode(raw)
    except Exception:
        return jsonify({"error": "Bad input"}), 400
    expected = encrypt(_KNOWN_PAYLOAD)
    match = ct == expected
    return jsonify({"match": match})  # vuln-code-snippet vuln-line chain_predictable_iv_vuln
# vuln-code-snippet end chain_predictable_iv_vuln
