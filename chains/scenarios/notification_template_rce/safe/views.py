"""Flask views for notification_template_rce scenario -- SAFE variant.

POST /notify passes message and user as context variables to
renderer.render_notification. The static template escapes values;
Jinja2 expressions in the message cannot execute.

Chain broken: message is context value -> static template -> no SSTI
"""
from flask import Blueprint, request, jsonify
from renderer import render_notification

notify_bp = Blueprint("notify", __name__)


# vuln-code-snippet start chain_notification_ssti_safe
@notify_bp.route("/notify", methods=["POST"])
def notify():
    """Send a notification with the message treated as a safe context variable."""
    message = request.json.get("message", "")
    user = request.json.get("user", "anonymous")
    rendered = render_notification(message, user)  # vuln-code-snippet safe-line chain_notification_ssti_safe
    return jsonify({"notification": rendered})
# vuln-code-snippet end chain_notification_ssti_safe
