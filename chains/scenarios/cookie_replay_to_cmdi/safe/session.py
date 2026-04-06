"""Session validation module -- SAFE variant.

Sessions are stored with the originating client's fingerprint (IP address
and User-Agent) at creation time, and the fingerprint is verified on every
subsequent request. A replayed cookie from a different IP or User-Agent
is rejected with 401.

Chain: stolen cookie replayed from different IP -> fingerprint mismatch -> 401 (chain broken)
Individual findings: CMDi still present in server_control.py (high)
Chain finding: none -- session is bound to the original client fingerprint
"""
import secrets
import functools
import hmac
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

    SAFE: verifies that the current request's IP and User-Agent match
    the values recorded when the session was created.
    """
    if not token:
        return None
    session = _sessions.get(token)
    if session is None:
        return None
    current_ip = request.remote_addr
    current_ua = request.headers.get("User-Agent", "")
    # SAFE: reject if the request comes from a different fingerprint
    if current_ip != session["bound_ip"]:
        return None
    if not hmac.compare_digest(current_ua, session["bound_ua"]):
        return None
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
