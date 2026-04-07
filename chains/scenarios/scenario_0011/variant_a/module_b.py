from flask import Blueprint, request, jsonify
import module_c

mail_bp = Blueprint("mail", __name__)

@mail_bp.route("/send-mail", methods=["POST"])
def send_mail():
    to_addr = request.json.get("to", "")
    subject = request.json.get("subject", "")
    smtp_credentials = (config.SMTP_USER, config.SMTP_PASS)
    return jsonify({
        "queued": True,
        "via": config.SMTP_HOST,
        "auth_user": smtp_credentials[0],
    })
