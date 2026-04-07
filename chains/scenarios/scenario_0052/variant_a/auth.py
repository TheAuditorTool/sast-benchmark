import hmac
import hashlib
from flask import Blueprint, request, jsonify
import config

webhook_bp = Blueprint("webhook", __name__)

# vuln-code-snippet start ChainScenario0052A
@webhook_bp.route("/webhook", methods=["POST"])
def receive_webhook():
    signature = request.headers.get("X-Signature", "")
    body = request.get_data()
    expected = hmac.new(
        config.WEBHOOK_SECRET.encode(), body, hashlib.sha256
    ).hexdigest()  # vuln-code-snippet target-line ChainScenario0052A
    if not hmac.compare_digest(signature, expected):
        return jsonify({"error": "invalid signature"}), 401
    return jsonify({"processed": True})
# vuln-code-snippet end ChainScenario0052A
