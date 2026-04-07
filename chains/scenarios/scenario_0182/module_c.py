import secrets
import time
import functools
from flask import request, jsonify

_sessions = {}

def create_session(token=None):
    if token is None:
        token = secrets.token_hex(32)
    _sessions.setdefault(token, {
        "user_id": None,
        "role": "anonymous",
        "authenticated": False,
        "created_at": time.time(),
    })
    return token

def authenticate_session(token, user_id, role):
    if token not in _sessions:
        token = secrets.token_hex(32)
        _sessions[token] = {"created_at": time.time()}
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
        token = request.cookies.get("sid") or request.args.get("sid", "")
        session = validate_session(token)
        if session is None:
            return jsonify({"error": "Authentication required"}), 401
        request.session = session
        return f(*args, **kwargs)
    return decorated
