"""Authentication middleware for v1 and v2 API -- SAFE variant.

v1_require_admin now applies the same full admin check as v2_require_admin.
Requests to the v1 endpoint are subject to identical authentication and
authorization constraints as v2, closing the version bypass.

Chain: attacker calls /api/v1/ endpoint -> same auth applied -> 401/403 (chain broken)
"""
import functools
from flask import request, jsonify
from models import USERS


def get_current_user():
    """Resolve caller from X-User-Id header."""
    return USERS.get(request.headers.get("X-User-Id", ""))


def v2_require_admin(f):
    """v2 admin enforcement: verifies authentication and admin role."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        user = get_current_user()
        if user is None:
            return jsonify({"error": "Authentication required"}), 401
        if user.get("role") != "admin":
            return jsonify({"error": "Admin role required"}), 403
        request.current_user = user
        return f(*args, **kwargs)
    return decorated


def v1_require_admin(f):
    """v1 admin enforcement: SAFE -- identical checks to v2_require_admin."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        user = get_current_user()
        if user is None:
            return jsonify({"error": "Authentication required"}), 401
        if user.get("role") != "admin":
            return jsonify({"error": "Admin role required"}), 403
        request.current_user = user
        return f(*args, **kwargs)
    return decorated
