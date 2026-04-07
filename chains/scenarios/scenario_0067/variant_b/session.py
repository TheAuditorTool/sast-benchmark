import secrets
import functools
from flask import request, jsonify

_sessions = {}

def create_session(user_id, role):
    token = secrets.token_hex(32)
    _sessions[token] = {
        "user_id": user_id,
        "role": role,
        "bound_ip": request.remote_addr,
        "bound_ua": request.headers.get("User-Agent", ""),
    }
    return token

def validate_session(token):
    if not token:
        return None
    session = _sessions.get(token)
    if session is None:
        return None
    
    return session

def require_session(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        token = request.cookies.get("ops_session", "")
        session = validate_session(token)
        if session is None:
            return jsonify({"error": "Invalid or missing session"}), 401
        request.session = session
        return f(*args, **kwargs)
    return decorated
