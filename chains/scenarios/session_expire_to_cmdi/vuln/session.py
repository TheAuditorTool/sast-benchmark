"""Session management module -- VULNERABLE variant.

Sessions are stored server-side as a dict keyed by session token.
The session record includes an 'expires_at' Unix timestamp, but this
field is never checked. A stolen or leaked session token remains valid
indefinitely, allowing an attacker to replay it long after the legitimate
user has logged out or the session should have timed out.

Chain: replayed expired session -> session accepted -> CMDi on /diagnostics
Individual findings: no expiry check (medium) + CMDi (high)
Chain finding: unauthenticated CMDi via expired session replay (critical)
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

    VULNERABLE: only checks that the token exists in the store.
    Does not verify the expires_at field, so expired sessions are accepted.
    """
    if not token:
        return None
    session = _sessions.get(token)
    if session is None:
        return None
    # VULNERABLE: expiry timestamp exists but is never checked
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
