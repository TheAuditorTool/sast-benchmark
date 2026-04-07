from flask import Blueprint, request, jsonify
import config

mail_bp = Blueprint("mail", __name__)

# vuln-code-snippet start ChainScenario0011A
@mail_bp.route("/send-mail", methods=["POST"])
def send_mail():
    to_addr = request.json.get("to", "")
    subject = request.json.get("subject", "")
    smtp_credentials = (config.SMTP_USER, config.SMTP_PASS)  # vuln-code-snippet target-line ChainScenario0011A
    return jsonify({
        "queued": True,
        "via": config.SMTP_HOST,
        "auth_user": smtp_credentials[0],
    })
# vuln-code-snippet end ChainScenario0011A
