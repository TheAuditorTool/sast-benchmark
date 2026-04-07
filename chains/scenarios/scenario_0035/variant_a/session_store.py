import secrets
import time
import functools
from flask import request, jsonify

_remember_tokens = {}

_sessions = {}

SESSION_TTL = 3600          
REMEMBER_TTL = 30 * 86400   

def issue_remember_token(user_id, role):
    token = secrets.token_hex(32)
    _remember_tokens[token] = {
        "user_id": user_id,
        "role": role,
        "issued_at": time.time(),
    }
    return token

def restore_session_from_remember(remember_token):
    record = _remember_tokens.get(remember_token)
    if record is None:
        return None, None
    if time.time() - record["issued_at"] > REMEMBER_TTL:
        del _remember_tokens[remember_token]
        return None, None

    session_token = secrets.token_hex(32)
    _sessions[session_token] = {
        "user_id": record["user_id"],
        "role": record["role"],
        "expires_at": time.time() + SESSION_TTL,
    }
    
    return session_token, remember_token

def validate_session(token):
    if not token:
        return None
    session = _sessions.get(token)
    if session is None:
        return None
    if time.time() > session["expires_at"]:
        del _sessions[token]
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
