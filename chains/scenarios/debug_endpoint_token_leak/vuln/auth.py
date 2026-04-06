"""Authentication handler.

This file is IDENTICAL between vuln/ and safe/ variants.

Accepts a Bearer token and resolves the user from SESSION_STORE.
Because the vulnerable debug.py leaks all tokens, any attacker can
impersonate any user by calling GET /debug/sessions first.

CWE-200: Leaked session tokens enable account takeover.
Chain: GET /debug/sessions leaks token -> attacker uses token -> authenticated as victim
"""
import functools
from flask import request, jsonify
from config import SESSION_STORE
from debug import app


def _get_user_from_token():
    """Resolve user_id from the Authorization: Bearer <token> header."""
    auth = request.headers.get("Authorization", "")
    if not auth.startswith("Bearer "):
        return None
    token = auth[len("Bearer "):]
    return SESSION_STORE.get(token)


def login_required(f):
    """Require a valid Bearer token."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        user_id = _get_user_from_token()
        if user_id is None:
            return jsonify({"error": "Authentication required"}), 401
        request.current_user_id = user_id
        return f(*args, **kwargs)
    return decorated


@app.route("/api/me")
@login_required
def me():
    """Return the current user's ID."""
    return jsonify({"user_id": request.current_user_id})


@app.route("/api/admin/action", methods=["POST"])
@login_required
def admin_action():
    """Perform an admin action."""
    user_id = request.current_user_id
    if user_id != "admin":
        return jsonify({"error": "Forbidden"}), 403
    return jsonify({"status": "admin action performed"})


if __name__ == "__main__":
    app.run(port=5000)
