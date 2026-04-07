import secrets
import time
import functools
from flask import request, jsonify

_sessions = {}

SESSION_TTL = 3600

def create_session(user_id, role, client_ip):
    token = secrets.token_hex(32)
    _sessions[token] = {
        "user_id": user_id,
        "role": role,
        "last_seen": time.time(),
        "ip": client_ip,
        "created_at": time.time(),
    }
    return token

def touch_session(token, client_ip):
    session = _sessions.get(token)
    if session is None:
        return None
    if time.time() - session["created_at"] > SESSION_TTL:
        del _sessions[token]
        return None
    session["last_seen"] = time.time()
    return session

def validate_session(token):
    if not token:
        return None
    session = _sessions.get(token)
    if session is None:
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
