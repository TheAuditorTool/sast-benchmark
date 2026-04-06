"""Auth endpoints for deterministic_nonce scenario -- SAFE variant.

POST /sign signs a message with an RFC 6979 nonce.
GET  /verify verifies the signature and grants access.

This file is IDENTICAL between vuln/ and safe/ variants (only
crypto.py changes).
"""
from flask import Blueprint, request, jsonify
from crypto import sign, verify

auth_bp = Blueprint("auth", __name__)


@auth_bp.route("/sign", methods=["POST"])
def do_sign():
    """Sign a message with an RFC 6979-derived nonce."""
    message = (request.json or {}).get("message", "")
    result = sign(message)
    return jsonify(result)


# vuln-code-snippet start chain_deterministic_nonce_safe
@auth_bp.route("/verify")
def do_verify():
    """Verify an RFC 6979 ECDSA signature and return protected data."""
    message = request.args.get("message", "")
    try:
        r = int(request.args.get("r", "0"))
        s = int(request.args.get("s", "0"))
    except ValueError:
        return jsonify({"error": "Bad parameters"}), 400
    if not verify(message, r, s):
        return jsonify({"error": "Invalid signature"}), 403
    return jsonify({"status": "verified", "message": message})  # vuln-code-snippet safe-line chain_deterministic_nonce_safe
# vuln-code-snippet end chain_deterministic_nonce_safe
