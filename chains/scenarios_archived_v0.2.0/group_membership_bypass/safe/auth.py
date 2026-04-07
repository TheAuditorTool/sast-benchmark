"""Group membership authorization -- SAFE variant.

check_group_access() now verifies that the user's group_ids list
actually contains the requested group_id before granting access.

Chain: user supplies grp-confidential -> membership verified -> 403 if not member (chain broken)
"""
import functools
from flask import request, jsonify
from models import USERS, GROUPS


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


def check_group_access(user_id, group_id):
    """Return True only if the group exists AND the user is a member.

    SAFE: membership is verified against the server-side USERS store.
    """
    if group_id not in GROUPS:
        return False
    user = USERS.get(user_id)
    if user is None:
        return False
    return group_id in user.get("group_ids", [])
