"""Auth endpoints for rsa_512_sign scenario -- SAFE variant.

POST /sign signs a payload with HMAC-SHA256.
GET  /verify verifies an HMAC-SHA256 signature.

This file is IDENTICAL between vuln/ and safe/ variants (only
crypto.py changes).
"""
from flask import Blueprint, request, jsonify
from crypto import sign, verify

auth_bp = Blueprint("auth", __name__)


@auth_bp.route("/sign", methods=["POST"])
def do_sign():
    """Sign the supplied message with HMAC-SHA256."""
    message = (request.json or {}).get("message", "")
    return jsonify({"message": message, "signature": sign(message)})


# vuln-code-snippet start chain_rsa_512_safe
@auth_bp.route("/verify")
def do_verify():
    """Verify an HMAC-SHA256 signature and grant access if valid."""
    message = request.args.get("message", "")
    signature = request.args.get("signature", "")
    if not verify(message, signature):
        return jsonify({"error": "Invalid signature"}), 403
    return jsonify({"status": "verified", "message": message})  # vuln-code-snippet safe-line chain_rsa_512_safe
# vuln-code-snippet end chain_rsa_512_safe
