"""Session management module -- SAFE variant.

The Secure cookie flag is set to True, so the browser will never send the
session cookie over plain HTTP.  Additionally, promote_session() regenerates
the token at login time, invalidating any pre-login token an attacker may
have observed.

Chain: /init issues Secure cookie -> cookie not sent over HTTP -> attacker
       cannot read it -> even if they could, new token issued at login
       (CWE-384 mitigated)
Individual findings: none
Chain finding: none -- Secure flag and token regeneration prevent fixation
"""
import secrets
import time
import functools
from flask import request, jsonify

_sessions = {}


def create_session(user_id=None, role="anonymous"):
    """Create a new session and return its token."""
    token = secrets.token_hex(32)
    _sessions[token] = {
        "user_id": user_id,
        "role": role,
        "authenticated": user_id is not None,
        "created_at": time.time(),
    }
    return token


def promote_session(token, user_id, role):
    """Issue a fresh token for the authenticated session, invalidating the old one.

    SAFE: the pre-login token is deleted and a new token is created, so any
    attacker who observed the original cookie gains nothing.
    """
    if token in _sessions:
        del _sessions[token]

    new_token = secrets.token_hex(32)
    _sessions[new_token] = {
        "user_id": user_id,
        "role": role,
        "authenticated": True,
        "created_at": time.time(),
    }
    return new_token


def get_secure_flag():
    """Return whether the Secure cookie flag should be set.

    SAFE: returns True, ensuring the cookie is never sent over HTTP.
    """
    return True


def validate_session(token):
    """Return session data for an authenticated token, else None."""
    if not token:
        return None
    session = _sessions.get(token)
    if session is None or not session.get("authenticated"):
        return None
    return session


def require_auth(f):
    """Decorator enforcing an authenticated session."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        token = request.cookies.get("session", "")
        session = validate_session(token)
        if session is None:
            return jsonify({"error": "Authentication required"}), 401
        request.session = session
        return f(*args, **kwargs)
    return decorated
