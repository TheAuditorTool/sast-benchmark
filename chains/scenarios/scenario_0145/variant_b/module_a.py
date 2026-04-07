import functools
from flask import request, jsonify
from module_b import USERS

def get_current_user():
    return USERS.get(request.headers.get("X-User-Id", ""))

def v2_require_admin(f):
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
