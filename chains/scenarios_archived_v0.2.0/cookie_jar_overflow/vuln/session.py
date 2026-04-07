"""Session management module -- VULNERABLE variant.

The application sets a new session cookie on every anonymous request to any
endpoint, without a per-domain cookie count limit.  An attacker controlling
a subdomain (or executing XSS) can flood the victim's cookie jar with
attacker-crafted cookies for the domain; browsers evict the oldest cookies
when the per-domain limit (typically 50) is reached.  The attacker times the
flood so that the legitimate session cookie is evicted and a new anonymous
request causes the victim's browser to accept an attacker-controlled session
token, which the attacker can then authenticate (CWE-384 via cookie eviction).

Chain: attacker floods domain cookies -> legitimate session evicted -> victim
       gets fresh anonymous session cookie (attacker-known) -> victim logs in ->
       attacker reuses the fixed token
Individual findings: unbounded session cookie creation (low)
Chain finding: cookie jar overflow enables session fixation (high)
"""
import secrets
import time
import functools
from flask import request, jsonify

_sessions = {}

SESSION_TTL = 3600
MAX_ANON_SESSIONS = 10000  # VULNERABLE: effectively unlimited


def get_or_create_session():
    """Return the current session token or create a new anonymous one.

    VULNERABLE: always creates a new session if no valid cookie is present,
    without any rate-limiting.  An attacker who floods the cookie jar knows
    exactly which token will be assigned next (they planted it).
    """
    token = request.cookies.get("session", "")
    if token and token in _sessions:
        session = _sessions[token]
        if time.time() - session["created_at"] < SESSION_TTL:
            return token, session

    new_token = secrets.token_hex(32)
    _sessions[new_token] = {
        "user_id": None,
        "role": "anonymous",
        "authenticated": False,
        "created_at": time.time(),
    }
    return new_token, _sessions[new_token]


def promote_session(token, user_id, role):
    """Authenticate an existing session in place without regeneration.

    VULNERABLE: does not rotate the token, so an attacker who planted the
    cookie via overflow fixation gains authenticated access.
    """
    if token not in _sessions:
        token = secrets.token_hex(32)
        _sessions[token] = {"created_at": time.time()}
    _sessions[token].update({
        "user_id": user_id,
        "role": role,
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
    if time.time() - session["created_at"] > SESSION_TTL:
        del _sessions[token]
        return None
    return session


def require_auth(f):
    """Decorator enforcing a valid authenticated session."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        token = request.cookies.get("session", "")
        session = validate_session(token)
        if session is None:
            return jsonify({"error": "Authentication required"}), 401
        request.session = session
        return f(*args, **kwargs)
    return decorated
