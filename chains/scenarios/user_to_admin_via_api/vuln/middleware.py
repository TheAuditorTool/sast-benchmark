"""Request authentication middleware -- VULNERABLE variant.

Verifies that a session token maps to a known user but does NOT enforce
role requirements. Any authenticated user can reach admin-only routes
because require_admin() is never called from the route layer.

Chain: authenticated low-priv user -> missing RBAC middleware -> admin action
Individual findings: missing authorization check (CWE-862)
Chain finding: user-to-admin privilege escalation
"""
import functools
from flask import request, jsonify
from models import USERS


def get_current_user():
    """Resolve the caller from the X-User-Id header. Returns user dict or None."""
    user_id = request.headers.get("X-User-Id", "")
    return USERS.get(user_id)


def require_login(f):
    """Decorator: rejects unauthenticated requests, but does not check role."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        user = get_current_user()
        if user is None:
            return jsonify({"error": "Authentication required"}), 401
        request.current_user = user
        return f(*args, **kwargs)
    return decorated


def require_admin(f):
    """Decorator: enforces admin role.

    VULNERABLE: this decorator exists but is never applied to the admin
    routes in routes.py -- only require_login is used there.
    """
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
