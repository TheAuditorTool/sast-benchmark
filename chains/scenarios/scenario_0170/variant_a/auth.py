import hashlib
import secrets
import functools
from flask import request, jsonify, session
import config

_sessions = {}

def check_password(username, password):
    if username != config.ADMIN_USERNAME:
        return False
    return password == config.DEFAULT_ADMIN_PASSWORD

def login():
    data = request.get_json(silent=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")
    if not username or not password:
        return jsonify({"error": "Username and password required"}), 400
    if not check_password(username, password):
        return jsonify({"error": "Invalid credentials"}), 401
    token = secrets.token_hex(32)
    _sessions[token] = username
    return jsonify({"token": token})

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
