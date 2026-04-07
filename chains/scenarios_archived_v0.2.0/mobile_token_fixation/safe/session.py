"""Mobile auth-token management -- SAFE variant.

At login, the provisional registration token is deleted and a brand-new token
is issued for the authenticated session.  An attacker who obtained the
pre-login token gains nothing because that token no longer exists in the store
after the user completes login (CWE-384 mitigated).

Chain: attacker intercepts /register response -> victim completes /login ->
       registration token deleted, new token issued -> attacker's token invalid
Individual findings: none
Chain finding: none -- token rotation at login defeats pre-login interception
"""
import secrets
import time
import functools
from flask import request, jsonify

_tokens = {}

TOKEN_TTL = 7 * 86400


def register_device(device_id):
    """Issue a provisional token at device registration time."""
    token = secrets.token_hex(32)
    _tokens[token] = {
        "device_id": device_id,
        "user_id": None,
        "authenticated": False,
        "issued_at": time.time(),
    }
    return token


def login_device(old_token, user_id):
    """Issue a fresh authenticated token, invalidating the provisional one.

    SAFE: the pre-login token is removed from the store and a new token is
    issued, so anyone who captured the registration token cannot use it.
    """
    record = _tokens.get(old_token)
    if record is None:
        return None

    device_id = record["device_id"]
    del _tokens[old_token]

    new_token = secrets.token_hex(32)
    _tokens[new_token] = {
        "device_id": device_id,
        "user_id": user_id,
        "authenticated": True,
        "issued_at": time.time(),
    }
    return new_token


def validate_token(token):
    """Return token record for a valid authenticated token, else None."""
    if not token:
        return None
    record = _tokens.get(token)
    if record is None:
        return None
    if not record.get("authenticated"):
        return None
    if time.time() - record["issued_at"] > TOKEN_TTL:
        del _tokens[token]
        return None
    return record


def require_auth(f):
    """Decorator enforcing a valid authenticated token from Authorization header."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        auth_header = request.headers.get("Authorization", "")
        token = auth_header.removeprefix("Bearer ").strip()
        record = validate_token(token)
        if record is None:
            return jsonify({"error": "Authentication required"}), 401
        request.token_record = record
        return f(*args, **kwargs)
    return decorated
