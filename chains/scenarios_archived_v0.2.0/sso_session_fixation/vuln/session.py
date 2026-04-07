"""Session management module for SSO callback -- VULNERABLE variant.

When the SSO identity provider redirects back to the application, the app
accepts a session token that was placed in the pre-SSO state cookie and
promotes it to an authenticated session in place (no regeneration).  An
attacker who can set a cookie on the victim's browser before the SSO flow
(e.g. via an older subdomain or XSS) can fix the session token and hijack
the account once the victim completes SSO login (CWE-384).

Chain: attacker sets pre-SSO cookie -> victim completes SSO -> same token
       promoted to authenticated -> attacker replays token
Individual findings: SSO callback does not regenerate session (medium)
Chain finding: pre-SSO token fixation leads to post-SSO account takeover (critical)
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


def complete_sso_session(token, user_id, role, email):
    """Mark a session as authenticated after SSO callback.

    VULNERABLE: promotes the token that arrived in the cookie (which could
    have been planted by an attacker) without issuing a new token.
    """
    if token not in _sessions:
        token = secrets.token_hex(32)
        _sessions[token] = {"created_at": time.time()}
    _sessions[token].update({
        "user_id": user_id,
        "role": role,
        "email": email,
        "authenticated": True,
    })
    return token


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
