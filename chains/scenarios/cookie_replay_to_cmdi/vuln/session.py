"""Session validation module -- VULNERABLE variant.

Sessions are stored with the originating client's fingerprint (IP address
and User-Agent) at creation time, but the fingerprint is never verified on
subsequent requests. A stolen session cookie can be replayed from any
machine without triggering a rejection.

Chain: stolen cookie replayed from different IP -> fingerprint not checked -> CMDi
Individual findings: no session binding (medium) + CMDi (high)
Chain finding: cookie theft enabling CMDi from arbitrary origin (critical)
"""
import secrets
import functools
from flask import request, jsonify

# Session store: {token: {user_id, role, bound_ip, bound_ua}}
_sessions = {}


def create_session(user_id, role):
    """Create a session bound to the current client fingerprint."""
    token = secrets.token_hex(32)
    _sessions[token] = {
        "user_id": user_id,
        "role": role,
        "bound_ip": request.remote_addr,
        "bound_ua": request.headers.get("User-Agent", ""),
    }
    return token


def validate_session(token):
    """Validate a session token. Returns session dict or None.

    VULNERABLE: checks that the session token exists but does not verify
    the bound_ip or bound_ua fields against the current request.
    """
    if not token:
        return None
    session = _sessions.get(token)
    if session is None:
        return None
    # VULNERABLE: bound_ip and bound_ua stored but never compared
    return session


def require_session(f):
    """Decorator that validates the ops_session cookie."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        token = request.cookies.get("ops_session", "")
        session = validate_session(token)
        if session is None:
            return jsonify({"error": "Invalid or missing session"}), 401
        request.session = session
        return f(*args, **kwargs)
    return decorated
