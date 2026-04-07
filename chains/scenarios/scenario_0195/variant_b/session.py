import secrets
import time
import functools
from flask import request, jsonify

_sessions = {}

def get_or_create_session(token=None):
    if token and token in _sessions:
        return token, _sessions[token]
    new_token = secrets.token_hex(32)
    _sessions[new_token] = {
        "user_id": None,
        "role": "anonymous",
        "authenticated": False,
        "created_at": time.time(),
    }
    return new_token, _sessions[new_token]

def promote_session(token, user_id, role):
    if token not in _sessions:
        token = secrets.token_hex(32)
        _sessions[token] = {}
    _sessions[token].update({
        "user_id": user_id,
        "role": role,
        "authenticated": True,
    })
    return token

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
        token = request.cookies.get("session_token", "")
        session = validate_session(token)
        if session is None:
            return jsonify({"error": "Authentication required"}), 401
        request.session = session
        return f(*args, **kwargs)
    return decorated
