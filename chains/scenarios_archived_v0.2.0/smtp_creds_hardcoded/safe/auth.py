"""Email sending handler for smtp_creds_hardcoded scenario -- SAFE variant.

POST /send-mail authenticates to SMTP using credentials loaded from
environment variables. No password is present in source.

Chain broken: config.SMTP_PASS from env -> no hardcoded credential -> no unauthorized relay
"""
from flask import Blueprint, request, jsonify
import config

mail_bp = Blueprint("mail", __name__)


# vuln-code-snippet start chain_smtp_creds_safe
@mail_bp.route("/send-mail", methods=["POST"])
def send_mail():
    """Queue an email using SMTP credentials loaded from the environment."""
    to_addr = request.json.get("to", "")
    subject = request.json.get("subject", "")
    smtp_credentials = (config.SMTP_USER, config.SMTP_PASS)  # vuln-code-snippet safe-line chain_smtp_creds_safe
    return jsonify({
        "queued": True,
        "via": config.SMTP_HOST,
        "auth_user": smtp_credentials[0],
    })
# vuln-code-snippet end chain_smtp_creds_safe
