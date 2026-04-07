"""Password reset flow -- IDENTICAL between vuln/ and safe/ variants.

Implements a two-step password reset: request a token (sent to the registered
email) and then submit the token with a new password. This module is the second
link in the chain: if registration.py allowed an attacker to register a variant
of a victim's email (e.g. victim+attacker@example.com), the reset token will
be issued to whichever account's email matches -- meaning the attacker can reset
the password of the victim's effective email address.

Chain: attacker registers victim+tag@domain (same normalized address as victim) ->
       POST /auth/password-reset/request finds attacker account -> token sent to
       attacker email -> attacker submits token -> victim account password changed
Individual findings: none in isolation (reset flow is correct given unique emails)
Chain finding: combined with email normalization bypass, enables full account
               takeover (critical, CWE-287)
"""
import secrets
import time
from flask import Blueprint, request, jsonify

reset_bp = Blueprint("reset", __name__)

# Populated by app.py to share the registration module's store
USER_STORE: dict[str, dict] = {}

_reset_tokens: dict[str, dict] = {}
_TOKEN_TTL_SECONDS = 900


def _send_reset_email(email: str, token: str) -> None:
    """Simulate sending a password reset email (logs to stdout in test mode)."""
    print(f"[RESET EMAIL] To: {email} Token: {token}")


@reset_bp.route("/auth/password-reset/request", methods=["POST"])
def request_reset():
    """Issue a password reset token for the given email address."""
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
    """Apply a new password using a valid reset token."""
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
