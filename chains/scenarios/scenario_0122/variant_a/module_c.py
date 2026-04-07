import secrets
import time
import functools
from flask import request, jsonify

_sessions = {}

COOKIE_DOMAIN = ".example.com"  

def create_session(user_id, role):
    token = secrets.token_hex(32)
    _sessions[token] = {
        "user_id": user_id,
        "role": role,
        "created_at": time.time(),
    }
    return token

def validate_session(token):
    if not token:
        return None
    return _sessions.get(token)

def get_cookie_domain():
    return COOKIE_DOMAIN

def require_auth(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        token = request.cookies.get("session", "")
        session = validate_session(token)
        if session is None:
            return jsonify({"error": "Authentication required"}), 401
        request.session = session
        return f(*args, **kwargs)
    return decorated
