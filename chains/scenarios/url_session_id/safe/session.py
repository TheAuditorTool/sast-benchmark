"""Session management module -- SAFE variant.

Session tokens are ONLY accepted from the HttpOnly cookie, never from URL
parameters.  Attacker-supplied sid values in query strings are ignored.
Additionally, authenticate_session() always issues a fresh token, invalidating
the pre-login token.

Chain: attacker crafts URL with known sid -> victim logs in -> URL sid ignored,
       new token issued -> fixation attempt fails (CWE-384 mitigated)
Individual findings: none
Chain finding: none -- URL sid ignored and token regenerated
"""
import secrets
import time
import functools
from flask import request, jsonify

_sessions = {}


def create_session(token=None):
    """Create a new server-generated session, ignoring any caller-supplied token."""
    new_token = secrets.token_hex(32)
    _sessions[new_token] = {
        "user_id": None,
        "role": "anonymous",
        "authenticated": False,
        "created_at": time.time(),
    }
    return new_token


def authenticate_session(token, user_id, role):
    """Invalidate the old token and return a fresh authenticated session token.

    SAFE: the old token is deleted unconditionally; a brand-new token is
    created so any previously fixed or URL-supplied token is worthless.
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


def validate_session(token):
    """Return session data for a valid authenticated token, else None."""
    if not token:
        return None
    session = _sessions.get(token)
    if session is None or not session.get("authenticated"):
        return None
    return session


def require_auth(f):
    """Decorator that accepts session token from cookie ONLY -- never from URL."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        token = request.cookies.get("sid", "")
        session = validate_session(token)
        if session is None:
            return jsonify({"error": "Authentication required"}), 401
        request.session = session
        return f(*args, **kwargs)
    return decorated
