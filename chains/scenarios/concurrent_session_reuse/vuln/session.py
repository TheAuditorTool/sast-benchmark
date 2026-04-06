"""Session management module -- VULNERABLE variant.

There is no limit on how many concurrent sessions a single user account may
have.  When a session token is stolen (via XSS, network sniffing, etc.) both
the legitimate user and the attacker can use the same session indefinitely.
Neither is evicted, and the server cannot distinguish legitimate from stolen
access, so the compromise persists until the token expires or the user logs
out from every device (CWE-384 / concurrent session abuse).

Chain: session token stolen -> both attacker and victim use same token ->
       no eviction, no detection -> attacker has persistent authenticated access
Individual findings: no concurrent session limit (low/medium)
Chain finding: stolen session co-used without detection enables persistent hijack (high)
"""
import secrets
import time
import functools
from flask import request, jsonify

# {token: {user_id, role, last_seen, ip}}
_sessions = {}

SESSION_TTL = 3600


def create_session(user_id, role, client_ip):
    """Create a new session without checking for existing active sessions.

    VULNERABLE: creates a new token regardless of how many sessions the user
    already has, and does not implement any last-writer-wins or single-session
    enforcement that would invalidate a concurrently used stolen token.
    """
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
    """Update last_seen for an active session.

    VULNERABLE: does not compare client_ip against the recorded IP, so an
    attacker using the token from a different IP goes undetected.
    """
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
