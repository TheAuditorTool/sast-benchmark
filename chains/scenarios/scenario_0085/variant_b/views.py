from flask import Blueprint, request, jsonify
from renderer import render_email

email_bp = Blueprint("email", __name__)

# vuln-code-snippet start ChainScenario0085B
@email_bp.route("/send-email", methods=["POST"])
def send_email():
    subject = request.json.get("subject", "No Subject")
    recipient = request.json.get("recipient", "user@example.com")
    body = render_email(subject, recipient)  # vuln-code-snippet target-line ChainScenario0085B
    return jsonify({"queued": True, "preview": body})
# vuln-code-snippet end ChainScenario0085B
