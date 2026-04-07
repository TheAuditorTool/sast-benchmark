import functools
from flask import g, request, jsonify
from module_c import is_session_valid

def load_session():
    token = request.headers.get("X-Session-Token", "")
    
    g.session_valid = is_session_valid(token)
    g.session_token = token

def require_valid_session(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        if not getattr(g, "session_valid", False):
            return jsonify({"error": "Invalid or expired session"}), 401

        return f(*args, **kwargs)

    return decorated
