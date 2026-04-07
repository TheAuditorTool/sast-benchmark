"""Auth endpoints for crc32_integrity scenario -- SAFE variant.

POST /send signs and stores a message with HMAC-SHA256.
GET  /receive retrieves and verifies the most recent message.

This file is IDENTICAL between vuln/ and safe/ variants (only
crypto.py changes).
"""
from flask import Blueprint, request, jsonify
from crypto import sign_message, verify_message

auth_bp = Blueprint("auth", __name__)

_INBOX: list = []


@auth_bp.route("/send", methods=["POST"])
def send():
    """HMAC-sign and store a message."""
    payload = (request.json or {}).get("message", "")
    _INBOX.append(sign_message(payload))
    return jsonify({"status": "stored"})


# vuln-code-snippet start chain_crc32_integrity_safe
@auth_bp.route("/receive")
def receive():
    """Retrieve and verify the latest message (HMAC-SHA256)."""
    if not _INBOX:
        return jsonify({"error": "empty"}), 404
    signed = _INBOX[-1]
    payload = verify_message(signed)
    if payload is None:
        return jsonify({"error": "integrity check failed"}), 400
    return jsonify({"message": payload})  # vuln-code-snippet safe-line chain_crc32_integrity_safe
# vuln-code-snippet end chain_crc32_integrity_safe
