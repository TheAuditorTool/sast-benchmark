import hashlib
import secrets
import functools
from flask import request, jsonify

_users = {
    "admin": hashlib.sha256(b"Adm1n$2026!").hexdigest(),
    "ops": hashlib.sha256(b"0ps_P@ss!").hexdigest(),
}

_sessions = {}

def login():
    data = request.get_json(silent=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")
    if not username or not password:
        return jsonify({"error": "Username and password required"}), 400
    expected = _users.get(username)
    if expected is None or expected != hashlib.sha256(password.encode()).hexdigest():
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
