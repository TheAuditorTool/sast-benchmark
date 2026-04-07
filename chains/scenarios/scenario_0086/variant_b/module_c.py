import time
import secrets
from flask import request, jsonify
import functools

_sessions = {}

def create_session(user_id, role, ttl_seconds=3600):
    token = secrets.token_hex(32)
    _sessions[token] = {
        "user_id": user_id,
        "role": role,
        "expires_at": time.time() + ttl_seconds,
    }
    return token

def validate_session(token):
    if not token:
        return None
    session = _sessions.get(token)
    if session is None:
        return None
    
    if time.time() > session["expires_at"]:
        return None
    return session

def require_session(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        token = request.cookies.get("session_token", "")
        session = validate_session(token)
        if session is None:
            return jsonify({"error": "Invalid or missing session"}), 401
        request.session = session
        return f(*args, **kwargs)
    return decorated
