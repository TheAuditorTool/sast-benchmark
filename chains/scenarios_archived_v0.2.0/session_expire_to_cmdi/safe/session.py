"""Session management module -- SAFE variant.

Sessions are stored server-side as a dict keyed by session token.
The session record includes an 'expires_at' Unix timestamp which is
checked on every request. Expired sessions are rejected with 401,
preventing replay attacks using leaked or stolen old tokens.

Chain: replayed expired session -> expiry check fails -> 401 (chain broken)
Individual findings: CMDi still present in diagnostics.py (high)
Chain finding: none -- expired token cannot reach the injection
"""
import time
import secrets
from flask import request, jsonify
import functools

# In-memory session store: {token: {user_id, role, expires_at}}
_sessions = {}


def create_session(user_id, role, ttl_seconds=3600):
    """Create a new session and return the session token."""
    token = secrets.token_hex(32)
    _sessions[token] = {
        "user_id": user_id,
        "role": role,
        "expires_at": time.time() + ttl_seconds,
    }
    return token


def validate_session(token):
    """Validate a session token. Returns the session dict or None.

    SAFE: verifies that expires_at has not passed before accepting the session.
    """
    if not token:
        return None
    session = _sessions.get(token)
    if session is None:
        return None
    # SAFE: reject sessions that have passed their expiry timestamp
    if time.time() > session["expires_at"]:
        return None
    return session


def require_session(f):
    """Decorator that validates the session cookie."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        token = request.cookies.get("session_token", "")
        session = validate_session(token)
        if session is None:
            return jsonify({"error": "Invalid or missing session"}), 401
        request.session = session
        return f(*args, **kwargs)
    return decorated
