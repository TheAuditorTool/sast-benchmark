import functools
from flask import request, jsonify
from module_b import USERS

ROLE_RANK = {"viewer": 1, "editor": 2, "admin": 3}

def get_current_user():
    return USERS.get(request.headers.get("X-User-Id", ""))

def require_viewer(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        user = get_current_user()
        if user is None:
            return jsonify({"error": "Authentication required"}), 401
        request.current_user = user
        return f(*args, **kwargs)
    return decorated

def require_editor(f):
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
