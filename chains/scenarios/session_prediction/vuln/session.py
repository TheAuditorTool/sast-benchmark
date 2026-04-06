"""Session management module -- VULNERABLE variant.

Session tokens are generated using a sequential counter combined with a
simple hash of a low-entropy value.  An attacker who registers two accounts
and observes the assigned tokens can deduce the pattern and predict tokens
assigned to other users -- including admins (CWE-384 / CWE-330).

Chain: attacker registers two accounts, observes sequential token IDs ->
       predicts token of another user -> sends predicted token ->
       gains access to victim account
Individual findings: predictable session token (high)
Chain finding: token prediction leads to unauthenticated account access (critical)
"""
import hashlib
import time
import functools
from flask import request, jsonify

_sessions = {}
_counter = 0  # VULNERABLE: global counter makes tokens sequential


def create_session(user_id, role):
    """Generate a predictable session token using a sequential counter.

    VULNERABLE: the token is derived from a monotonically increasing counter
    and a fixed salt, making it enumerable given two known tokens.
    """
    global _counter
    _counter += 1
    raw = f"session-{_counter}-{user_id}"
    token = hashlib.md5(raw.encode()).hexdigest()  # noqa: S324 -- intentionally weak
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
