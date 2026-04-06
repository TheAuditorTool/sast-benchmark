"""Session management module for SSO callback -- SAFE variant.

complete_sso_session() ALWAYS discards the pre-SSO token and issues a fresh
cryptographically random token.  Even if an attacker planted a cookie before
the SSO flow, that token is invalidated the moment the victim completes
authentication, so the attacker gains nothing (CWE-384 mitigated).

Chain: attacker plants sso_session cookie -> victim completes SSO ->
       planted token deleted, new token issued -> attacker's cookie is stale
Individual findings: none
Chain finding: none -- token regeneration defeats pre-SSO fixation
"""
import secrets
import time
import functools
from flask import request, jsonify

_sessions = {}


def create_pre_sso_session():
    """Create an anonymous session before SSO redirect."""
    token = secrets.token_hex(32)
    _sessions[token] = {
        "user_id": None,
        "role": "anonymous",
        "authenticated": False,
        "created_at": time.time(),
    }
    return token


def complete_sso_session(old_token, user_id, role, email):
    """Invalidate the pre-SSO token and return a fresh authenticated session token.

    SAFE: the old token is unconditionally deleted before creating the new
    authenticated session, preventing any fixed token from surviving the
    SSO transition.
    """
    if old_token in _sessions:
        del _sessions[old_token]

    new_token = secrets.token_hex(32)
    _sessions[new_token] = {
        "user_id": user_id,
        "role": role,
        "email": email,
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
    """Decorator enforcing an authenticated session."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        token = request.cookies.get("sso_session", "")
        session = validate_session(token)
        if session is None:
            return jsonify({"error": "Authentication required"}), 401
        request.session = session
        return f(*args, **kwargs)
    return decorated
