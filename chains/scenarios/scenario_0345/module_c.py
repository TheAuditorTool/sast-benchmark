import re
from flask import Blueprint, request, jsonify

reg_bp = Blueprint("reg", __name__)

USER_STORE: dict[str, dict] = {}

_EMAIL_RE = re.compile(r'^[^@\s]+@[^@\s]+\.[^@\s]+$')

@reg_bp.route("/auth/register", methods=["POST"])
def register():
    body = request.get_json(silent=True)
    if not body:
        return jsonify({"error": "JSON body required"}), 400

    email = body.get("email", "").strip()
    password = body.get("password", "")

    if not email or not _EMAIL_RE.match(email):
        return jsonify({"error": "Valid email address required"}), 400
    if not password or len(password) < 8:
        return jsonify({"error": "Password must be at least 8 characters"}), 400

    if email in USER_STORE:
        return jsonify({"error": "Email already registered"}), 409

    USER_STORE[email] = {"email": email, "password": password}
    return jsonify({"status": "registered", "email": email}), 201
