"""Flask views for email_template_ssti scenario -- VULNERABLE variant.

POST /send-email accepts a JSON body with 'subject' and 'recipient' fields.
The subject is embedded directly into a Jinja2 template string and rendered,
allowing SSTI-based RCE via injected Jinja2 expressions.

Chain: POST /send-email -> renderer.render_email(subject) -> from_string -> RCE
"""
from flask import Blueprint, request, jsonify
from renderer import render_email

email_bp = Blueprint("email", __name__)


# vuln-code-snippet start chain_email_ssti_vuln
@email_bp.route("/send-email", methods=["POST"])
def send_email():
    """Render and queue an email with the user-supplied subject."""
    subject = request.json.get("subject", "No Subject")
    recipient = request.json.get("recipient", "user@example.com")
    body = render_email(subject, recipient)  # vuln-code-snippet vuln-line chain_email_ssti_vuln
    return jsonify({"queued": True, "preview": body})
# vuln-code-snippet end chain_email_ssti_vuln
