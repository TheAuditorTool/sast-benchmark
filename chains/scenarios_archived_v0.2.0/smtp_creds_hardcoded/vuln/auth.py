"""Email sending handler for smtp_creds_hardcoded scenario -- VULNERABLE variant.

POST /send-mail authenticates to the SMTP server using the hardcoded
credentials from config. An attacker who reads the source can send
arbitrary email as the application account.

Chain: POST /send-mail -> config.SMTP_PASS -> SMTP login -> unauthorized relay
"""
from flask import Blueprint, request, jsonify
import config

mail_bp = Blueprint("mail", __name__)


# vuln-code-snippet start chain_smtp_creds_vuln
@mail_bp.route("/send-mail", methods=["POST"])
def send_mail():
    """Queue an email using the hardcoded SMTP credentials."""
    to_addr = request.json.get("to", "")
    subject = request.json.get("subject", "")
    smtp_credentials = (config.SMTP_USER, config.SMTP_PASS)  # vuln-code-snippet vuln-line chain_smtp_creds_vuln
    return jsonify({
        "queued": True,
        "via": config.SMTP_HOST,
        "auth_user": smtp_credentials[0],
    })
# vuln-code-snippet end chain_smtp_creds_vuln
