"""Mobile auth-token management -- VULNERABLE variant.

An auth token is generated at device registration and reused as-is after the
user subsequently logs in on the same device.  Because the token is set before
credentials are verified and never rotated after a successful login, an attacker
who intercepts the registration response can use the same token to act as the
authenticated user once they log in (CWE-384).

Chain: device registers -> token issued -> user logs in -> same token valid ->
       attacker (who saw registration token) now has authenticated access
Individual findings: auth token not rotated after login (medium)
Chain finding: pre-login token fixation gives attacker post-login access (high)
"""
import secrets
import time
import functools
from flask import request, jsonify

# {token: {device_id, user_id, authenticated, issued_at}}
_tokens = {}

TOKEN_TTL = 7 * 86400  # 7-day token lifetime


def register_device(device_id):
    """Issue a provisional token at device registration time.

    VULNERABLE: this token is the same one that will be used after login;
    if an attacker sees it they can wait for the user to authenticate and
    then use it.
    """
    token = secrets.token_hex(32)
    _tokens[token] = {
        "device_id": device_id,
        "user_id": None,
        "authenticated": False,
        "issued_at": time.time(),
    }
    return token


def login_device(token, user_id):
    """Bind an existing device token to an authenticated user.

    VULNERABLE: the token value is not rotated -- authentication simply
    flips the 'authenticated' flag on the pre-registration token.
    """
    record = _tokens.get(token)
    if record is None:
        return None
    record["user_id"] = user_id
    record["authenticated"] = True
    return token


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
