"""Session management module -- VULNERABLE variant.

Sessions are keyed by a token generated at first visit (pre-authentication).
After the user logs in, the same token continues to be used without regeneration.
An attacker who knows the pre-login token (e.g. via network sniffing on HTTP or
a link the victim followed) can wait for the victim to authenticate and then use
the same token as an authenticated session -- a textbook session fixation attack.

Chain: attacker plants known session token -> victim logs in -> attacker reuses
       token as authenticated session (CWE-384)
Individual findings: missing session regeneration (medium)
Chain finding: unauthenticated access via fixed session token (critical)
"""
import secrets
import time
import functools
from flask import request, jsonify

# In-memory session store: {token: {user_id, role, authenticated, created_at}}
_sessions = {}


def get_or_create_session(token=None):
    """Return an existing session or create a new anonymous one.

    VULNERABLE: re-uses the caller-supplied token directly without
    validating whether it was created by this server.  An attacker can
    therefore supply an arbitrary token value and have it promoted to an
    authenticated session once the victim logs in.
    """
    if token and token in _sessions:
        return token, _sessions[token]
    new_token = secrets.token_hex(32)
    _sessions[new_token] = {
        "user_id": None,
        "role": "anonymous",
        "authenticated": False,
        "created_at": time.time(),
    }
    return new_token, _sessions[new_token]


def promote_session(token, user_id, role):
    """Promote an existing session to authenticated.

    VULNERABLE: the token is NOT regenerated here.  The pre-login token
    is kept, so any attacker who planted that token gains authenticated access.
    """
    if token not in _sessions:
        token = secrets.token_hex(32)
        _sessions[token] = {}
    _sessions[token].update({
        "user_id": user_id,
        "role": role,
        "authenticated": True,
    })
    return token


def validate_session(token):
    """Return the session dict if the token belongs to an authenticated session."""
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
        token = request.cookies.get("session_token", "")
        session = validate_session(token)
        if session is None:
            return jsonify({"error": "Authentication required"}), 401
        request.session = session
        return f(*args, **kwargs)
    return decorated
