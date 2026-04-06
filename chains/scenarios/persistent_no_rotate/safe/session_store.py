"""Persistent session (remember-me) store -- SAFE variant.

The remember-me token is ROTATED on every use: the old token is deleted and a
fresh cryptographically random token is issued.  An attacker who steals a
remember-me token from a past breach or network capture has a one-shot window
at best; once the legitimate user uses the token, the stolen value is invalid.

Chain: remember-me token stolen -> replayed -> REJECTED if legitimate user
       has already used it (token rotated) (CWE-384 mitigated)
Individual findings: none
Chain finding: none -- rotation limits stolen-token window to one use
"""
import secrets
import time
import functools
from flask import request, jsonify

_remember_tokens = {}
_sessions = {}

SESSION_TTL = 3600
REMEMBER_TTL = 30 * 86400


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
    """Exchange a remember-me token for a new short session, rotating the token.

    SAFE: the old remember_token is deleted immediately and a new one is
    returned.  Each token is single-use, limiting the damage from theft.
    """
    record = _remember_tokens.get(remember_token)
    if record is None:
        return None, None
    if time.time() - record["issued_at"] > REMEMBER_TTL:
        del _remember_tokens[remember_token]
        return None, None

    user_id = record["user_id"]
    role = record["role"]

    del _remember_tokens[remember_token]

    new_remember = secrets.token_hex(32)
    _remember_tokens[new_remember] = {
        "user_id": user_id,
        "role": role,
        "issued_at": time.time(),
    }

    session_token = secrets.token_hex(32)
    _sessions[session_token] = {
        "user_id": user_id,
        "role": role,
        "expires_at": time.time() + SESSION_TTL,
    }
    return session_token, new_remember


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
