"""Login and session management -- SAFE variant.

The login endpoint rejects the default admin password ('admin') even if it
matches the configured value. The admin must change the password before
gaining access. An attacker using admin/admin credentials is blocked.

Chain: default creds login attempt -> rejected with 403 -> chain broken
Individual findings: SQLi still present in routes.py (medium)
Chain finding: none -- default credentials are not accepted
"""
import secrets
import functools
from flask import request, jsonify
import config

# In-memory session store: {token: user_id}
_sessions = {}


def check_password(username, password):
    """Verify admin credentials.

    SAFE: rejects the default password, forcing a password change before
    the admin account can be used for the first time.
    """
    if username != config.ADMIN_USERNAME:
        return False
    # SAFE: explicitly block the well-known default password
    if password == config.DEFAULT_ADMIN_PASSWORD:
        return False
    return password == config.DEFAULT_ADMIN_PASSWORD  # unreachable if default blocked above


def login():
    """Handle POST /auth/login."""
    data = request.get_json(silent=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")
    if not username or not password:
        return jsonify({"error": "Username and password required"}), 400
    if password == config.DEFAULT_ADMIN_PASSWORD:
        return jsonify({"error": "Default password must be changed before first login"}), 403
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
