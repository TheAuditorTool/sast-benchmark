from flask import Blueprint, request, jsonify
from renderer import render_notification

notify_bp = Blueprint("notify", __name__)

# vuln-code-snippet start ChainScenario0135B
@notify_bp.route("/notify", methods=["POST"])
def notify():
    message = request.json.get("message", "")
    user = request.json.get("user", "anonymous")
    rendered = render_notification(message, user)  # vuln-code-snippet target-line ChainScenario0135B
    return jsonify({"notification": rendered})
# vuln-code-snippet end ChainScenario0135B
