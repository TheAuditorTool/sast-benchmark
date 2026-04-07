import secrets
import time
import functools
from flask import request, jsonify

_sessions = {}

def create_session(user_id=None, role="anonymous"):
    token = secrets.token_hex(32)
    _sessions[token] = {
        "user_id": user_id,
        "role": role,
        "authenticated": user_id is not None,
        "created_at": time.time(),
    }
    return token

def promote_session(token, user_id, role):
    if token in _sessions:
        del _sessions[token]

    new_token = secrets.token_hex(32)
    _sessions[new_token] = {
        "user_id": user_id,
        "role": role,
        "authenticated": True,
        "created_at": time.time(),
    }
    return new_token

def get_secure_flag():
    return True

def validate_session(token):
    if not token:
        return None
    session = _sessions.get(token)
    if session is None or not session.get("authenticated"):
        return None
    return session

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
