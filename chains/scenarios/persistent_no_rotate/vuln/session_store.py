"""Persistent session (remember-me) store -- VULNERABLE variant.

A long-lived remember-me token is issued at first login and stored permanently.
The token is NEVER rotated -- every time it is used to restore a session, the
same value is re-issued in the cookie.  An attacker who obtains the token once
(database breach, cookie theft) can use it indefinitely without detection.

Chain: remember-me token issued -> never rotated -> stolen token replayed
       months later -> full account access (CWE-384 / persistent session abuse)
Individual findings: non-rotating persistent token (medium)
Chain finding: stolen remember-me token provides permanent account access (high)
"""
import secrets
import time
import functools
from flask import request, jsonify

# {remember_token: {user_id, role, issued_at}}
_remember_tokens = {}
# {session_token: {user_id, role, expires_at}}
_sessions = {}

SESSION_TTL = 3600          # 1-hour short session
REMEMBER_TTL = 30 * 86400   # 30-day persistent token


def issue_remember_token(user_id, role):
    """Create a persistent remember-me token for the user."""
    token = secrets.token_hex(32)
    _remember_tokens[token] = {
        "user_id": user_id,
        "role": role,
        "issued_at": time.time(),
    }
    return token


def restore_session_from_remember(remember_token):
    """Exchange a remember-me token for a new short session.

    VULNERABLE: the same remember_token is kept in the store without rotation.
    An attacker who steals the token can use it repeatedly without the server
    ever detecting or invalidating it.
    """
    record = _remember_tokens.get(remember_token)
    if record is None:
        return None, None
    if time.time() - record["issued_at"] > REMEMBER_TTL:
        del _remember_tokens[remember_token]
        return None, None

    session_token = secrets.token_hex(32)
    _sessions[session_token] = {
        "user_id": record["user_id"],
        "role": record["role"],
        "expires_at": time.time() + SESSION_TTL,
    }
    # VULNERABLE: remember_token is not rotated here
    return session_token, remember_token


def validate_session(token):
    """Return session data if the short-lived session token is valid."""
    if not token:
        return None
    session = _sessions.get(token)
    if session is None:
        return None
    if time.time() > session["expires_at"]:
        del _sessions[token]
        return None
    return session


def require_auth(f):
    """Decorator enforcing a valid session, auto-restoring from remember-me."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        token = request.cookies.get("session_token", "")
        session = validate_session(token)
        if session is None:
            return jsonify({"error": "Authentication required"}), 401
        request.session = session
        return f(*args, **kwargs)
    return decorated
