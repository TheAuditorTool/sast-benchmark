"""Notification service -- VULNERABLE variant.

Fetches an HTML email template from a user-supplied URL and sends it via the
internal SMTP relay. An attacker can supply a URL pointing to the SMTP relay
itself (using the gopher:// scheme for raw TCP, or http:// for header
injection), bypassing the network trust boundary to send arbitrary email.

Chain: attacker -> /notify?template_url=gopher://smtp-relay.internal:25/... -> open relay
"""
from flask import Flask, request, jsonify
import urllib.request
import urllib.error

app = Flask(__name__)


# vuln-code-snippet start chain_ssrf_smtp_vuln
def send_notification():
    """Fetch an email template and enqueue it for delivery."""
    template_url = request.args.get("template_url", "")
    recipient = request.args.get("recipient", "")
    if not template_url or not recipient:
        return jsonify({"error": "template_url and recipient parameters required"}), 400
    # No URL validation -- accepts any scheme including gopher:// for cross-protocol attacks
    try:
        response = urllib.request.urlopen(template_url)  # vuln-code-snippet vuln-line chain_ssrf_smtp_vuln
        template_body = response.read(65536)
        # Simulate enqueuing the notification -- template content is trusted
        return jsonify({"status": "queued", "recipient": recipient, "template_size": len(template_body)})
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502
# vuln-code-snippet end chain_ssrf_smtp_vuln


app.add_url_rule("/api/notify", view_func=send_notification)

if __name__ == "__main__":
    app.run(port=5016)
