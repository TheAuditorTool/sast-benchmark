"""Session management module -- SAFE variant.

The session token is ONLY accepted from the HttpOnly cookie.  The form POST
body field is completely ignored.  Even if an attacker extracts the token
value from a hidden form field, they cannot use it to authenticate because
require_auth never reads from form data.

Chain: token in hidden field -> XSS reads DOM -> token replayed -> REJECTED
       because require_auth only reads HttpOnly cookie (CWE-384 mitigated)
Individual findings: none
Chain finding: none -- token in form field cannot authenticate requests
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
    """Decorator that reads the session token from the HttpOnly cookie ONLY.

    SAFE: form body fields are never consulted, so a token extracted from
    a DOM hidden field cannot be used to bypass authentication.
    """
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        token = request.cookies.get("session_token", "")
        session = validate_session(token)
        if session is None:
            return jsonify({"error": "Authentication required"}), 401
        request.session = session
        return f(*args, **kwargs)
    return decorated
