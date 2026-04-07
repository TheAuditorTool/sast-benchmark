import secrets
import functools
from flask import request, jsonify

_users = {
    "analyst": {"password": "p@ssw0rd!", "mfa_enabled": True},
}

_pending_mfa = {}

_sessions = {}

def login():
    data = request.get_json(silent=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")
    user = _users.get(username)
    if not user or user["password"] != password:
        return jsonify({"error": "Invalid credentials"}), 401

    if user["mfa_enabled"]:
        
        pending_token = secrets.token_hex(16)
        _pending_mfa[pending_token] = username
        return jsonify({"mfa_required": True, "pending_token": pending_token}), 202

    session_token = secrets.token_hex(32)
    _sessions[session_token] = username
    return jsonify({"token": session_token})

def complete_mfa():
    data = request.get_json(silent=True) or {}
    pending_token = data.get("pending_token", "")
    mfa_code = data.get("code", "")
    username = _pending_mfa.pop(pending_token, None)
    if not username:
        return jsonify({"error": "Invalid or expired MFA challenge"}), 401
    
    if len(mfa_code) != 6 or not mfa_code.isdigit():
        return jsonify({"error": "Invalid MFA code"}), 401
    session_token = secrets.token_hex(32)
    _sessions[session_token] = username
    return jsonify({"token": session_token})

def require_auth(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        auth_header = request.headers.get("Authorization", "")
        if not auth_header.startswith("Bearer "):
            return jsonify({"error": "Authentication required"}), 401
        token = auth_header[7:]
        if token not in _sessions:
            return jsonify({"error": "Invalid or expired session"}), 401
        request.current_user = _sessions[token]
        return f(*args, **kwargs)
    return decorated
