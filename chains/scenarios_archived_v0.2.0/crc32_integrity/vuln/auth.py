"""Auth endpoints for crc32_integrity scenario.

POST /send signs and stores a message.
GET  /receive retrieves and verifies the most recent message.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Blueprint, request, jsonify
from crypto import sign_message, verify_message

auth_bp = Blueprint("auth", __name__)

_INBOX: list = []


@auth_bp.route("/send", methods=["POST"])
def send():
    """Sign and store a message."""
    payload = (request.json or {}).get("message", "")
    _INBOX.append(sign_message(payload))
    return jsonify({"status": "stored"})


# vuln-code-snippet start chain_crc32_integrity_vuln
@auth_bp.route("/receive")
def receive():
    """Retrieve and verify the latest message."""
    if not _INBOX:
        return jsonify({"error": "empty"}), 404
    signed = _INBOX[-1]
    payload = verify_message(signed)
    if payload is None:
        return jsonify({"error": "integrity check failed"}), 400
    return jsonify({"message": payload})  # vuln-code-snippet vuln-line chain_crc32_integrity_vuln
# vuln-code-snippet end chain_crc32_integrity_vuln
