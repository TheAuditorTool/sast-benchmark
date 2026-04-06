"""Session management module -- SAFE variant.

When a user logs in, ALL existing sessions for that user are invalidated before
the new session is created.  This implements single-active-session enforcement:
a stolen session token is automatically evicted when the legitimate user logs
in again, terminating the attacker's access (CWE-384 mitigated).

Chain: session stolen -> attacker uses token -> victim re-authenticates ->
       all old tokens for that user deleted -> attacker's token rejected
Individual findings: none
Chain finding: none -- login evicts all prior sessions, killing stolen token
"""
import secrets
import time
import functools
from flask import request, jsonify

_sessions = {}

SESSION_TTL = 3600


def create_session(user_id, role, client_ip):
    """Invalidate all sessions for user_id then create a fresh one.

    SAFE: deleting all previous sessions for the user means that a stolen
    token is invalidated as soon as the legitimate user logs in again.
    """
    stale = [t for t, s in _sessions.items() if s.get("user_id") == user_id]
    for t in stale:
        del _sessions[t]

    token = secrets.token_hex(32)
    _sessions[token] = {
        "user_id": user_id,
        "role": role,
        "last_seen": time.time(),
        "ip": client_ip,
        "created_at": time.time(),
    }
    return token


def touch_session(token, client_ip):
    """Update last_seen for an active session."""
    session = _sessions.get(token)
    if session is None:
        return None
    if time.time() - session["created_at"] > SESSION_TTL:
        del _sessions[token]
        return None
    session["last_seen"] = time.time()
    return session


def validate_session(token):
    """Return session data for a valid token, else None."""
    if not token:
        return None
    session = _sessions.get(token)
    if session is None:
        return None
    if time.time() - session["created_at"] > SESSION_TTL:
        del _sessions[token]
        return None
    return session


def require_auth(f):
    """Decorator enforcing a valid session cookie."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        token = request.cookies.get("session", "")
        session = validate_session(token)
        if session is None:
            return jsonify({"error": "Authentication required"}), 401
        request.session = session
        return f(*args, **kwargs)
    return decorated
