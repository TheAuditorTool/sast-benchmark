"""Session and impersonation authentication -- SAFE variant.

resolve_identity() ignores the X-User-Id header entirely.
Identity is determined solely from the server-side session token lookup,
which an attacker cannot forge without a valid session.

Chain: attacker sets X-User-Id: adm-001 -> header ignored -> 401 without valid session
"""
import functools
from flask import request, jsonify
from models import USERS, SUPPORT_SESSIONS


def resolve_identity():
    """Return the effective user dict for this request.

    SAFE: X-User-Id header is not honoured. Only the session token is used.
    """
    session_token = request.headers.get("X-Session-Token", "")
    user_id = SUPPORT_SESSIONS.get(session_token)
    if user_id:
        return USERS.get(user_id)
    return None


def require_admin(f):
    """Reject non-admin callers."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        user = resolve_identity()
        if user is None:
            return jsonify({"error": "Authentication required"}), 401
        if user.get("role") != "admin":
            return jsonify({"error": "Admin role required"}), 403
        request.current_user = user
        return f(*args, **kwargs)
    return decorated
