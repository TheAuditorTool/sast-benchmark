"""Session management module -- VULNERABLE variant.

The session cookie is set with domain='.example.com' (leading dot), making it
readable by every subdomain including attacker-controlled ones (e.g.
evil.example.com).  A malicious subdomain can read the cookie directly or
inject JavaScript to steal it, then replay it against the main application.

Chain: broad cookie domain -> subdomain reads session cookie ->
       replays token against app.example.com (CWE-384 / cookie scope issue)
Individual findings: overly broad cookie domain (medium)
Chain finding: cross-subdomain session theft leads to account takeover (high)
"""
import secrets
import time
import functools
from flask import request, jsonify

_sessions = {}

COOKIE_DOMAIN = ".example.com"  # VULNERABLE: covers all subdomains


def create_session(user_id, role):
    """Create an authenticated session token."""
    token = secrets.token_hex(32)
    _sessions[token] = {
        "user_id": user_id,
        "role": role,
        "created_at": time.time(),
    }
    return token


def validate_session(token):
    """Return the session dict for a valid token, else None."""
    if not token:
        return None
    return _sessions.get(token)


def get_cookie_domain():
    """Return the configured cookie domain.

    VULNERABLE: returns the broad parent-domain value that allows any
    subdomain to read the session cookie.
    """
    return COOKIE_DOMAIN


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
