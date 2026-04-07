import functools
from flask import request, jsonify
from module_b import USERS

def get_current_user():
    return USERS.get(request.headers.get("X-User-Id", ""))

def require_login(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        user = get_current_user()
        if user is None:
            return jsonify({"error": "Authentication required"}), 401
        request.current_user = user
        return f(*args, **kwargs)
    return decorated
