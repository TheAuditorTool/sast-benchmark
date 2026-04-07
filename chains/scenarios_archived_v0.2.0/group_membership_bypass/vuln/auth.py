"""Group membership authorization -- VULNERABLE variant.

check_group_access() verifies membership by checking whether the
user-supplied group_id is in the GROUPS dict, but does NOT verify that
the current user is actually a member of that group. Any authenticated
user can pass any group_id to access its documents.

Chain: low-priv user -> supplies arbitrary group_id -> accesses confidential docs
Individual findings: missing authorization check (CWE-862)
Chain finding: group membership check bypassed via parameter tampering
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
    """Return True if the group exists.

    VULNERABLE: does not verify that user_id belongs to the group.
    """
    return group_id in GROUPS
