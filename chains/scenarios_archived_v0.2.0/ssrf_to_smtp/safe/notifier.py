"""Notification service -- SAFE variant.

Template URLs are restricted to HTTPS scheme and must come from the known
template hosting domain. This blocks gopher://, http://, and any host that
is not the authorised template server, breaking the cross-protocol SMTP chain.
"""
from flask import Flask, request, jsonify
import urllib.request
import urllib.error
from urllib.parse import urlparse

app = Flask(__name__)

ALLOWED_TEMPLATE_HOSTS = {"templates.example.com"}
REQUIRED_SCHEME = "https"


# vuln-code-snippet start chain_ssrf_smtp_safe
def send_notification():
    """Fetch an email template and enqueue it for delivery."""
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
        response = urllib.request.urlopen(template_url)  # vuln-code-snippet safe-line chain_ssrf_smtp_safe
        template_body = response.read(65536)
        return jsonify({"status": "queued", "recipient": recipient, "template_size": len(template_body)})
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502
# vuln-code-snippet end chain_ssrf_smtp_safe


app.add_url_rule("/api/notify", view_func=send_notification)

if __name__ == "__main__":
    app.run(port=5016)
