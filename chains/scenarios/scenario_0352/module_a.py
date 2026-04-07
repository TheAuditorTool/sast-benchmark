import functools
from flask import session, jsonify

def get_current_user_id():
    return session.get("user_id")

def login_required(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        if get_current_user_id() is None:
            return jsonify({"error": "Authentication required"}), 401
        return f(*args, **kwargs)
    return decorated
