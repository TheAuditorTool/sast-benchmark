"""Flask views for email_template_ssti scenario -- SAFE variant.

POST /send-email passes subject and recipient as context variables to
renderer.render_email. The template is static and autoescaped; Jinja2
expressions in the subject cannot execute.

Chain broken: subject is context value -> static template -> no SSTI
"""
from flask import Blueprint, request, jsonify
from renderer import render_email

email_bp = Blueprint("email", __name__)


# vuln-code-snippet start chain_email_ssti_safe
@email_bp.route("/send-email", methods=["POST"])
def send_email():
    """Render and queue an email with the subject treated as a safe context value."""
    subject = request.json.get("subject", "No Subject")
    recipient = request.json.get("recipient", "user@example.com")
    body = render_email(subject, recipient)  # vuln-code-snippet safe-line chain_email_ssti_safe
    return jsonify({"queued": True, "preview": body})
# vuln-code-snippet end chain_email_ssti_safe
