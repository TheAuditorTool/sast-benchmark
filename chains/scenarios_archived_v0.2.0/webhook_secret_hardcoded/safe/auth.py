"""Webhook handler for webhook_secret_hardcoded scenario -- SAFE variant.

POST /webhook verifies signatures using a secret loaded from an
environment variable. No secret is embedded in source.

Chain broken: config.WEBHOOK_SECRET from env -> no hardcoded secret -> no forgery
"""
import hmac
import hashlib
from flask import Blueprint, request, jsonify
import config

webhook_bp = Blueprint("webhook", __name__)


# vuln-code-snippet start chain_webhook_secret_safe
@webhook_bp.route("/webhook", methods=["POST"])
def receive_webhook():
    """Verify and process an incoming webhook using an environment-sourced signing secret."""
    signature = request.headers.get("X-Signature", "")
    body = request.get_data()
    expected = hmac.new(
        config.WEBHOOK_SECRET.encode(), body, hashlib.sha256
    ).hexdigest()  # vuln-code-snippet safe-line chain_webhook_secret_safe
    if not hmac.compare_digest(signature, expected):
        return jsonify({"error": "invalid signature"}), 401
    return jsonify({"processed": True})
# vuln-code-snippet end chain_webhook_secret_safe
