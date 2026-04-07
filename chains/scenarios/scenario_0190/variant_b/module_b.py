import functools
from flask import request, jsonify
from module_c import is_session_valid

def load_session():
    pass

def require_valid_session(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        token = request.headers.get("X-Session-Token", "")

        if not is_session_valid(token):
            return jsonify({"error": "Invalid or expired session"}), 401
        return f(*args, **kwargs)

    return decorated
