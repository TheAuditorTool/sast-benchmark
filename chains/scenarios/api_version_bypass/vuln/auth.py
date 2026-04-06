"""Authentication middleware for v1 and v2 API -- VULNERABLE variant.

v2_require_admin correctly enforces admin role for sensitive operations.
v1_require_admin was the original implementation but lacks authentication
entirely -- it was left active when v2 was deployed. Clients targeting
/api/v1/ can bypass authentication added in v2.

Chain: attacker calls /api/v1/ endpoint -> no auth on v1 -> privileged operation
Individual findings: missing authentication for critical function (CWE-287)
Chain finding: v1 API version bypass to unauthenticated admin action
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
    """Legacy v1 admin decorator -- VULNERABLE: performs no checks.

    This was a placeholder during early development and was never secured.
    The v1 prefix was meant to be deprecated but the route remains live.
    """
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        return f(*args, **kwargs)
    return decorated
