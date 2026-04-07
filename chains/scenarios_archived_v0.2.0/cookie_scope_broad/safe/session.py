"""Session management module -- SAFE variant.

The session cookie is set with domain=None (omitted), which causes the browser
to scope it to the exact host that issued it.  Subdomains cannot read the
cookie, breaking the cross-subdomain session theft chain.

Chain: exact-host cookie set at login -> subdomain cannot read cookie ->
       session cannot be stolen (CWE-384 mitigated)
Individual findings: none
Chain finding: none -- narrow cookie domain prevents subdomain read
"""
import secrets
import time
import functools
from flask import request, jsonify

_sessions = {}

COOKIE_DOMAIN = None  # SAFE: browser scopes cookie to the exact issuing host


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

    SAFE: returns None so the browser restricts the cookie to the exact host,
    preventing any subdomain from accessing the session token.
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
