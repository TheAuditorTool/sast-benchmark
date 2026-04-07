import secrets
import time
from flask import Blueprint, request, jsonify

reset_bp = Blueprint("reset", __name__)

USER_STORE: dict[str, dict] = {}

_reset_tokens: dict[str, dict] = {}
_TOKEN_TTL_SECONDS = 900

def _send_reset_email(email: str, token: str) -> None:
    print(f"[RESET EMAIL] To: {email} Token: {token}")

@reset_bp.route("/auth/password-reset/request", methods=["POST"])
def request_reset():
    body = request.get_json(silent=True) or {}
    email = body.get("email", "").strip()
    if not email:
        return jsonify({"error": "email required"}), 400

    user = USER_STORE.get(email)
    if user is None:
        return jsonify({"message": "If that address is registered, a reset link was sent"}), 200

    token = secrets.token_urlsafe(32)
    _reset_tokens[token] = {"email": email, "expires_at": time.time() + _TOKEN_TTL_SECONDS}
    _send_reset_email(email, token)
    return jsonify({"message": "If that address is registered, a reset link was sent"}), 200

@reset_bp.route("/auth/password-reset/confirm", methods=["POST"])
def confirm_reset():
    body = request.get_json(silent=True) or {}
    token = body.get("token", "")
    new_password = body.get("new_password", "")

    if not token or not new_password:
        return jsonify({"error": "token and new_password required"}), 400

    token_data = _reset_tokens.get(token)
    if token_data is None:
        return jsonify({"error": "Invalid or expired token"}), 400
    if time.time() > token_data["expires_at"]:
        _reset_tokens.pop(token, None)
        return jsonify({"error": "Token has expired"}), 400

    email = token_data["email"]
    user = USER_STORE.get(email)
    if user is None:
        return jsonify({"error": "User not found"}), 404

    user["password"] = new_password
    _reset_tokens.pop(token, None)
    return jsonify({"status": "password updated"}), 200
