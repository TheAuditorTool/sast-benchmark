"""Webhook handler for webhook_secret_hardcoded scenario -- VULNERABLE variant.

POST /webhook verifies the HMAC-SHA256 signature of incoming payloads
using the hardcoded secret. An attacker who knows the secret can forge
signatures for arbitrary payloads.

Chain: POST /webhook -> config.WEBHOOK_SECRET -> hmac verify -> forged event processed
"""
import hmac
import hashlib
from flask import Blueprint, request, jsonify
import config

webhook_bp = Blueprint("webhook", __name__)


# vuln-code-snippet start chain_webhook_secret_vuln
@webhook_bp.route("/webhook", methods=["POST"])
def receive_webhook():
    """Verify and process an incoming webhook using the hardcoded signing secret."""
    signature = request.headers.get("X-Signature", "")
    body = request.get_data()
    expected = hmac.new(
        config.WEBHOOK_SECRET.encode(), body, hashlib.sha256
    ).hexdigest()  # vuln-code-snippet vuln-line chain_webhook_secret_vuln
    if not hmac.compare_digest(signature, expected):
        return jsonify({"error": "invalid signature"}), 401
    return jsonify({"processed": True})
# vuln-code-snippet end chain_webhook_secret_vuln
