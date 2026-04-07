import functools
from flask import request, jsonify
from module_b import USERS, SUPPORT_SESSIONS

def resolve_identity():
    session_token = request.headers.get("X-Session-Token", "")
    user_id = SUPPORT_SESSIONS.get(session_token)
    if user_id:
        return USERS.get(user_id)
    return None

def require_admin(f):
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
