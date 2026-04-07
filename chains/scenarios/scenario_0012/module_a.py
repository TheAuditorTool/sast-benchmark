from flask import Blueprint, request, jsonify
from module_c import sign_message, verify_message

auth_bp = Blueprint("auth", __name__)

_INBOX: list = []

@auth_bp.route("/send", methods=["POST"])
def send():
    payload = (request.json or {}).get("message", "")
    _INBOX.append(sign_message(payload))
    return jsonify({"status": "stored"})

@auth_bp.route("/receive")
def receive():
    if not _INBOX:
        return jsonify({"error": "empty"}), 404
    signed = _INBOX[-1]
    payload = verify_message(signed)
    if payload is None:
        return jsonify({"error": "integrity check failed"}), 400
    return jsonify({"message": payload})
