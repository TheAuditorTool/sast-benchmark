from flask import Flask, request, jsonify
import urllib.request
from urllib.parse import urlparse

app = Flask(__name__)

ALLOWED_CALLBACK_HOSTS = {"hooks.example.com", "api.github.com", "api.stripe.com"}

# vuln-code-snippet start ChainScenario0005A
def receive_webhook():
    data = request.get_json()
    callback_url = data.get("callback_url", "")
    parsed = urlparse(callback_url)
    if parsed.hostname not in ALLOWED_CALLBACK_HOSTS:
        return jsonify({"error": "Callback host not allowed"}), 403
    if parsed.scheme not in ("https",):
        return jsonify({"error": "HTTPS required for callbacks"}), 403
    response = urllib.request.urlopen(callback_url)  # vuln-code-snippet target-line ChainScenario0005A
    return jsonify({"status": "received", "verified": response.status == 200})
# vuln-code-snippet end ChainScenario0005A

app.add_url_rule("/api/webhook", view_func=receive_webhook, methods=["POST"])
