"""Auth endpoints for rsa_512_sign scenario.

POST /sign signs a payload.
GET  /verify verifies a signature.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Blueprint, request, jsonify
from crypto import sign, verify

auth_bp = Blueprint("auth", __name__)


@auth_bp.route("/sign", methods=["POST"])
def do_sign():
    """Sign the supplied message and return the signature."""
    message = (request.json or {}).get("message", "")
    return jsonify({"message": message, "signature": sign(message)})


# vuln-code-snippet start chain_rsa_512_vuln
@auth_bp.route("/verify")
def do_verify():
    """Verify a message/signature pair and grant access if valid."""
    message = request.args.get("message", "")
    signature = request.args.get("signature", "")
    if not verify(message, signature):
        return jsonify({"error": "Invalid signature"}), 403
    return jsonify({"status": "verified", "message": message})  # vuln-code-snippet vuln-line chain_rsa_512_vuln
# vuln-code-snippet end chain_rsa_512_vuln
