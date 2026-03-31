"""Webhook receiver with SSRF -- IDENTICAL between vuln/ and safe/."""
from flask import Flask, request, jsonify
import urllib.request

app = Flask(__name__)

# vuln-code-snippet start chain_ssrf_admin_vuln
def receive_webhook():
    """Process incoming webhook. Validates payload by fetching callback URL."""
    data = request.get_json()
    callback_url = data.get("callback_url", "")
    # Fetch the callback URL to verify webhook authenticity
    # No validation on the URL -- can reach internal services
    response = urllib.request.urlopen(callback_url)  # vuln-code-snippet vuln-line chain_ssrf_admin_vuln
    return jsonify({"status": "received", "verified": response.status == 200})
# vuln-code-snippet end chain_ssrf_admin_vuln

app.add_url_rule("/api/webhook", view_func=receive_webhook, methods=["POST"])
