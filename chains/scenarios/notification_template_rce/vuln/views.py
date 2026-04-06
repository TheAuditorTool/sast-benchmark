"""Flask views for notification_template_rce scenario -- VULNERABLE variant.

POST /notify accepts a JSON body with 'message' and 'user' fields.
The message is embedded in a Jinja2 template string and rendered,
allowing SSTI-based RCE if the message contains Jinja2 expressions.

Chain: POST /notify -> renderer.render_notification(message) -> from_string -> RCE
"""
from flask import Blueprint, request, jsonify
from renderer import render_notification

notify_bp = Blueprint("notify", __name__)


# vuln-code-snippet start chain_notification_ssti_vuln
@notify_bp.route("/notify", methods=["POST"])
def notify():
    """Send a notification rendered from a Jinja2 template embedding the message."""
    message = request.json.get("message", "")
    user = request.json.get("user", "anonymous")
    rendered = render_notification(message, user)  # vuln-code-snippet vuln-line chain_notification_ssti_vuln
    return jsonify({"notification": rendered})
# vuln-code-snippet end chain_notification_ssti_vuln
