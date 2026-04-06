"""Session and impersonation authentication -- VULNERABLE variant.

resolve_identity() checks the X-User-Id header first. If present, it
uses that value as the acting user identity, enabling any caller who
knows a target user_id to impersonate them including admins.
The support session token is verified correctly, but is overridden by
the debug header.

Chain: attacker sets X-User-Id: adm-001 -> impersonates admin -> privileged action
Individual findings: improper privilege management (CWE-269)
Chain finding: debug header enables admin impersonation
"""
import functools
from flask import request, jsonify
from models import USERS, SUPPORT_SESSIONS


def resolve_identity():
    """Return the effective user dict for this request.

    VULNERABLE: honours X-User-Id header, allowing caller-controlled impersonation.
    """
    override_id = request.headers.get("X-User-Id", "")
    if override_id:
        return USERS.get(override_id)
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
