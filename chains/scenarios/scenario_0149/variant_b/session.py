import secrets
import time
import functools
from flask import request, jsonify

_sessions = {}

SESSION_TTL = 3600

def get_or_create_session():
    token = request.cookies.get("session", "")
    if token and token in _sessions:
        session = _sessions[token]
        if time.time() - session["created_at"] < SESSION_TTL:
            return token, session

    new_token = secrets.token_hex(32)
    _sessions[new_token] = {
        "user_id": None,
        "role": "anonymous",
        "authenticated": False,
        "created_at": time.time(),
    }
    return new_token, _sessions[new_token]

def promote_session(old_token, user_id, role):
    if old_token in _sessions:
        del _sessions[old_token]

    new_token = secrets.token_hex(32)
    _sessions[new_token] = {
        "user_id": user_id,
        "role": role,
        "authenticated": True,
        "created_at": time.time(),
    }
    return new_token

def validate_session(token):
    if not token:
        return None
    session = _sessions.get(token)
    if session is None or not session.get("authenticated"):
        return None
    if time.time() - session["created_at"] > SESSION_TTL:
        del _sessions[token]
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
