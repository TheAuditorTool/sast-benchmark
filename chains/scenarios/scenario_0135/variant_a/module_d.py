from flask import Blueprint, request, jsonify
from module_c import render_notification

notify_bp = Blueprint("notify", __name__)

@notify_bp.route("/notify", methods=["POST"])
def notify():
    message = request.json.get("message", "")
    user = request.json.get("user", "anonymous")
    rendered = render_notification(message, user)
    return jsonify({"notification": rendered})
