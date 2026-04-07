"""Profile update authorization -- VULNERABLE variant.

require_own_profile() verifies that the caller is editing their own profile
(IDOR protection). However, it does not filter which fields can be changed.
Combined with the route accepting a 'role' field, a user can escalate by
patching their own profile with role=manager.

Chain: user edits own profile (IDOR ok) -> role field not stripped -> vertical escalation
Individual findings: missing authorization check (CWE-862)
Chain finding: horizontal access + unrestricted field edit = vertical privilege escalation
"""
import functools
from flask import request, jsonify
from models import PROFILES


def get_current_user_id():
    """Resolve caller from X-User-Id header."""
    return request.headers.get("X-User-Id", "")


def require_own_profile(f):
    """Verify caller is editing their own profile only.

    VULNERABLE: does not restrict which fields may be updated.
    """
    @functools.wraps(f)
    def decorated(user_id, *args, **kwargs):
        caller_id = get_current_user_id()
        if not caller_id or caller_id not in PROFILES:
            return jsonify({"error": "Authentication required"}), 401
        if caller_id != user_id:
            return jsonify({"error": "Cannot edit another user's profile"}), 403
        request.current_user_id = caller_id
        return f(user_id, *args, **kwargs)
    return decorated
