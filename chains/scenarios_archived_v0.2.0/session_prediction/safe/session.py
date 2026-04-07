"""Session management module -- SAFE variant.

Session tokens are generated using secrets.token_hex(32), producing 256 bits
of cryptographically secure randomness.  No sequential counter or low-entropy
input is used, so tokens cannot be predicted even if the attacker observes
many valid tokens (CWE-384 / CWE-330 mitigated).

Chain: attacker observes two tokens -> cannot predict others ->
       brute-force infeasible at 256-bit token space
Individual findings: none
Chain finding: none -- unpredictable tokens defeat enumeration
"""
import secrets
import time
import functools
from flask import request, jsonify

_sessions = {}


def create_session(user_id, role):
    """Generate a cryptographically random session token.

    SAFE: uses secrets.token_hex(32) for 256 bits of entropy, making
    tokens computationally infeasible to predict or enumerate.
    """
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
