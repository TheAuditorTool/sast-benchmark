import secrets
import time
import functools
from flask import request, jsonify

_sessions = {}

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

def require_auth(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        token = (
            request.cookies.get("session_token")
            or request.form.get("session_token", "")
        )
        session = validate_session(token)
        if session is None:
            return jsonify({"error": "Authentication required"}), 401
        request.session = session
        return f(*args, **kwargs)
    return decorated
