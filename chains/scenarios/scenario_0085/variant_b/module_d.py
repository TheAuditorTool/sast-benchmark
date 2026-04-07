from flask import Blueprint, request, jsonify
from module_c import render_email

email_bp = Blueprint("email", __name__)

@email_bp.route("/send-email", methods=["POST"])
def send_email():
    subject = request.json.get("subject", "No Subject")
    recipient = request.json.get("recipient", "user@example.com")
    body = render_email(subject, recipient)
    return jsonify({"queued": True, "preview": body})
