"""Session management module -- VULNERABLE variant.

The session token is embedded in every HTML form as a hidden input field
instead of being confined to an HttpOnly cookie.  Because the token sits in
the DOM, any XSS payload or MITM attacker who can read page source can
extract it directly and replay it to hijack the session (CWE-384).

Chain: token in hidden field -> XSS / MITM reads field -> token replayed
Individual findings: session token exposed in DOM (medium)
Chain finding: DOM-extractable session token leads to account hijacking (high)
"""
import secrets
import time
import functools
from flask import request, jsonify

_sessions = {}


def create_session(user_id, role):
    """Create an authenticated session and return the token."""
    token = secrets.token_hex(32)
    _sessions[token] = {
        "user_id": user_id,
        "role": role,
        "created_at": time.time(),
    }
    return token


def validate_session(token):
    """Return session data for a valid token, else None."""
    if not token:
        return None
    return _sessions.get(token)


def require_auth(f):
    """Decorator that reads the session token from a form field or cookie.

    VULNERABLE: falls back to reading the token from the POST body field
    'session_token', which means it is present in every rendered form and
    readable by JavaScript / network observers.
    """
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        token = (
            request.cookies.get("session_token")
            or request.form.get("session_token", "")
        )
        session = validate_session(token)
        if session is None:
            return jsonify({"error": "Authentication required"}), 401
        request.session = session
        return f(*args, **kwargs)
    return decorated
