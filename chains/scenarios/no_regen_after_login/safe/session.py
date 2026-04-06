"""Session management module -- SAFE variant.

Sessions are keyed by a token generated at first visit (pre-authentication).
After the user logs in, promote_session() ALWAYS issues a brand-new token and
invalidates the old one.  An attacker who planted a known pre-login token gains
nothing because that token is deleted as soon as the victim authenticates.

Chain: attacker plants known session token -> victim logs in -> old token deleted,
       new token issued -> fixation attempt fails (chain broken, CWE-384 mitigated)
Individual findings: none
Chain finding: none -- regeneration prevents fixation
"""
import secrets
import time
import functools
from flask import request, jsonify

# In-memory session store: {token: {user_id, role, authenticated, created_at}}
_sessions = {}


def get_or_create_session(token=None):
    """Return an existing server-issued session or create a new anonymous one.

    Only tokens present in _sessions (created by this server) are accepted;
    arbitrary caller-supplied values are silently discarded.
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


def promote_session(old_token, user_id, role):
    """Promote a session to authenticated, issuing a fresh token.

    SAFE: the old token is deleted and a new cryptographically random token is
    returned, so any attacker who knew the pre-login token cannot reuse it.
    """
    if old_token in _sessions:
        del _sessions[old_token]

    new_token = secrets.token_hex(32)
    _sessions[new_token] = {
        "user_id": user_id,
        "role": role,
        "authenticated": True,
        "created_at": time.time(),
    }
    return new_token


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
