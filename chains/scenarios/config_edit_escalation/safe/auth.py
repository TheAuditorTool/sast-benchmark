"""Settings access control -- VULNERABLE variant.

require_login confirms the caller is authenticated but does not gate
admin-only feature flags. The route is expected to check flag.admin_only
before applying changes, but it does not, allowing any user to enable
system-level feature flags like maintenance_mode.

Chain: user -> no admin_only enforcement in route -> admin feature flag toggled
Individual findings: improper privilege management (CWE-269)
Chain finding: authenticated user can modify admin-only feature flags
"""
import functools
from flask import request, jsonify
from models import USERS


def get_current_user():
    """Resolve caller from X-User-Id header."""
    return USERS.get(request.headers.get("X-User-Id", ""))


def require_login(f):
    """Reject unauthenticated requests."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        user = get_current_user()
        if user is None:
            return jsonify({"error": "Authentication required"}), 401
        request.current_user = user
        return f(*args, **kwargs)
    return decorated
