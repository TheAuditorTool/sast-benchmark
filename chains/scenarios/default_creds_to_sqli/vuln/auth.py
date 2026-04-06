"""Login and session management -- VULNERABLE variant.

The login endpoint accepts the default admin password ('admin') without
requiring it to be changed on first use. An attacker can log in with
publicly known credentials (admin/admin) and obtain a valid session,
then reach the SQLi on the user management endpoints.

Chain: default creds login -> valid session -> SQLi on /admin/users
Individual findings: default credentials (high) + SQLi (medium)
Chain finding: unauthenticated SQLi via default creds (critical)
"""
import hashlib
import secrets
import functools
from flask import request, jsonify, session
import config

# In-memory session store: {token: user_id}
_sessions = {}


def check_password(username, password):
    """Verify admin credentials.

    VULNERABLE: accepts the default password without checking if it has
    been changed from the factory default. Any attacker who knows the
    default ('admin'/'admin') can authenticate.
    """
    if username != config.ADMIN_USERNAME:
        return False
    return password == config.DEFAULT_ADMIN_PASSWORD


def login():
    """Handle POST /auth/login."""
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
    """Decorator that validates the bearer session token."""
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
