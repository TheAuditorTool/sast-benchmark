"""Authorization helpers -- VULNERABLE variant.

require_viewer enforces that the caller is at least a viewer (any logged-in
user). require_editor exists but is not applied to the PATCH route, so a
viewer can mutate articles despite the intent.

Chain: viewer -> PATCH endpoint lacks role check -> content modified
Individual findings: missing authorization check (CWE-862)
Chain finding: viewer-to-editor privilege escalation
"""
import functools
from flask import request, jsonify
from models import USERS

ROLE_RANK = {"viewer": 1, "editor": 2, "admin": 3}


def get_current_user():
    """Resolve caller from X-User-Id header."""
    return USERS.get(request.headers.get("X-User-Id", ""))


def require_viewer(f):
    """Allow any authenticated user (viewer or above)."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        user = get_current_user()
        if user is None:
            return jsonify({"error": "Authentication required"}), 401
        request.current_user = user
        return f(*args, **kwargs)
    return decorated


def require_editor(f):
    """Require editor role or above.

    VULNERABLE: defined here but never applied to the PATCH route in routes.py.
    """
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        user = get_current_user()
        if user is None:
            return jsonify({"error": "Authentication required"}), 401
        if ROLE_RANK.get(user.get("role", ""), 0) < ROLE_RANK["editor"]:
            return jsonify({"error": "Editor role required"}), 403
        request.current_user = user
        return f(*args, **kwargs)
    return decorated
