import hmac
import hashlib
from flask import Blueprint, request, jsonify
import module_c

webhook_bp = Blueprint("webhook", __name__)

@webhook_bp.route("/webhook", methods=["POST"])
def receive_webhook():
    signature = request.headers.get("X-Signature", "")
    body = request.get_data()
    expected = hmac.new(
        config.WEBHOOK_SECRET.encode(), body, hashlib.sha256
    ).hexdigest()
    if not hmac.compare_digest(signature, expected):
        return jsonify({"error": "invalid signature"}), 401
    return jsonify({"processed": True})
