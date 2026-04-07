"""Session management module -- SAFE variant.

promote_session() always deletes the old token and issues a fresh
cryptographically random one.  Even if an attacker successfully evicts the
legitimate session via a cookie jar overflow and forces a new anonymous session,
the token is rotated at login so the attacker's knowledge of the pre-login token
is useless (CWE-384 mitigated).

Chain: cookie jar flooded -> cookie evicted -> victim gets new anonymous session ->
       victim logs in -> old token deleted, new token issued ->
       attacker cannot use the pre-login token they observed
Individual findings: none
Chain finding: none -- token rotation at login defeats overflow fixation
"""
import secrets
import time
import functools
from flask import request, jsonify

_sessions = {}

SESSION_TTL = 3600


def get_or_create_session():
    """Return the current session token or create a new anonymous one."""
    token = request.cookies.get("session", "")
    if token and token in _sessions:
        session = _sessions[token]
        if time.time() - session["created_at"] < SESSION_TTL:
            return token, session

    new_token = secrets.token_hex(32)
    _sessions[new_token] = {
        "user_id": None,
        "role": "anonymous",
        "authenticated": False,
        "created_at": time.time(),
    }
    return new_token, _sessions[new_token]


def promote_session(old_token, user_id, role):
    """Delete the old session and return a fresh authenticated token.

    SAFE: regardless of how the pre-login token was created or whether it was
    planted by an attacker via cookie overflow, it is deleted here and a new
    unpredictable token is issued.
    """
    if old_token in _sessions:
        del _sessions[old_token]

    new_token = secrets.token_hex(32)
    _sessions[new_token] = {
        "user_id": user_id,
        "role": role,
        "authenticated": True,
        "created_at": time.time(),
    }
    return new_token


def validate_session(token):
    """Return session data for a valid authenticated token, else None."""
    if not token:
        return None
    session = _sessions.get(token)
    if session is None or not session.get("authenticated"):
        return None
    if time.time() - session["created_at"] > SESSION_TTL:
        del _sessions[token]
        return None
    return session


def require_auth(f):
    """Decorator enforcing a valid authenticated session."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        token = request.cookies.get("session", "")
        session = validate_session(token)
        if session is None:
            return jsonify({"error": "Authentication required"}), 401
        request.session = session
        return f(*args, **kwargs)
    return decorated
