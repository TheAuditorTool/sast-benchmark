"""Password reset consumer.

This file is IDENTICAL between vuln/ and safe/ variants.

Accepts a reset token and new password.  Because debug.py returns the
token in the API response, an attacker can complete the reset without
email access.

CWE-200: Reset token in response enables account takeover without email access.
Chain: POST /password-reset/request returns token -> attacker uses token -> password changed
"""
import functools
from flask import request, jsonify
from config import app, USERS
from debug import request_reset  # noqa: F401 - registers /password-reset/request route


@app.route("/password-reset/confirm", methods=["POST"])
def confirm_reset():
    """Apply a password reset using the token."""
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    token = data.get("token", "")
    new_password = data.get("new_password", "")
    user = USERS.get(username)
    if user is None:
        return jsonify({"error": "User not found"}), 404
    if not token or user.get("reset_token") != token:
        return jsonify({"error": "Invalid or expired token"}), 401
    user["password"] = new_password
    user["reset_token"] = None
    return jsonify({"status": "password updated"})


if __name__ == "__main__":
    app.run(port=5000)
