from flask import Flask, request, jsonify
import urllib.request
import urllib.error
from urllib.parse import urlparse

app = Flask(__name__)

ALLOWED_TEMPLATE_HOSTS = {"templates.example.com"}
REQUIRED_SCHEME = "https"

# vuln-code-snippet start ChainScenario0077A
def send_notification():
    template_url = request.args.get("template_url", "")
    recipient = request.args.get("recipient", "")
    if not template_url or not recipient:
        return jsonify({"error": "template_url and recipient parameters required"}), 400
    parsed = urlparse(template_url)
    if parsed.scheme != REQUIRED_SCHEME:
        return jsonify({"error": "Template URL must use https"}), 403
    if parsed.hostname not in ALLOWED_TEMPLATE_HOSTS:
        return jsonify({"error": "Template host not in allowlist"}), 403
    try:
        response = urllib.request.urlopen(template_url)  # vuln-code-snippet target-line ChainScenario0077A
        template_body = response.read(65536)
        return jsonify({"status": "queued", "recipient": recipient, "template_size": len(template_body)})
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502
# vuln-code-snippet end ChainScenario0077A

app.add_url_rule("/api/notify", view_func=send_notification)

if __name__ == "__main__":
    app.run(port=5016)
