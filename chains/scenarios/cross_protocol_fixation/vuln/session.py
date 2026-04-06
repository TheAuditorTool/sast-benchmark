"""Session management module -- VULNERABLE variant.

A session created over plain HTTP is not invalidated when the same client
later connects over HTTPS.  Because the cookie lacks the Secure flag, it
was transmitted over HTTP where an on-path attacker could read it; the
session is then carried forward into the HTTPS context, giving the attacker
a persistent foothold even after the connection upgraded (CWE-384).

Chain: HTTP request sets session cookie (no Secure flag) -> attacker on
       network reads cookie -> HTTPS login proceeds with same token ->
       attacker replays cookie against HTTPS endpoint
Individual findings: missing Secure flag on session cookie (medium)
Chain finding: cross-protocol session fixation enables HTTPS session hijack (high)
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
    """Mark an existing session as authenticated without regenerating the token.

    VULNERABLE: the token survives the HTTP->HTTPS protocol transition so an
    attacker who read it over HTTP can reuse it over HTTPS.
    """
    if token in _sessions:
        _sessions[token].update({
            "user_id": user_id,
            "role": role,
            "authenticated": True,
        })
        return token
    return create_session(user_id, role)


def get_secure_flag():
    """Return whether the Secure cookie flag should be set.

    VULNERABLE: returns False, allowing the cookie to be sent over HTTP
    where it is visible to network attackers.
    """
    return False


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
