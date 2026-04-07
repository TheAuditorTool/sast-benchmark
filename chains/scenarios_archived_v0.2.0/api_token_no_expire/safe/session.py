"""API token management -- SAFE variant.

API tokens are issued with an expiry timestamp (90 days by default).
validate_api_token() checks the current time against expires_at and rejects
tokens that have passed their expiry, limiting the damage window from a leak.

Chain: token issued at registration -> leaked -> replayed after expiry ->
       REJECTED (token expired) (CWE-384 mitigated)
Individual findings: none
Chain finding: none -- expiry limits the usefulness of a leaked token
"""
import secrets
import time
import functools
from flask import request, jsonify

_api_tokens = {}

TOKEN_TTL = 90 * 86400  # 90-day expiry


def issue_api_token(user_id, role):
    """Generate a new API token with an expiry timestamp.

    SAFE: records expires_at so validate_api_token() can reject stale tokens.
    """
    token = secrets.token_hex(32)
    _api_tokens[token] = {
        "user_id": user_id,
        "role": role,
        "issued_at": time.time(),
        "expires_at": time.time() + TOKEN_TTL,
    }
    return token


def revoke_api_token(token):
    """Remove an API token from the store."""
    _api_tokens.pop(token, None)


def validate_api_token(token):
    """Return token record if the token is valid and has not expired.

    SAFE: rejects tokens whose expires_at timestamp has passed.
    """
    if not token:
        return None
    record = _api_tokens.get(token)
    if record is None:
        return None
    if time.time() > record["expires_at"]:
        del _api_tokens[token]
        return None
    return record


def require_api_auth(f):
    """Decorator enforcing a valid, non-expired API token from Authorization header."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        auth_header = request.headers.get("Authorization", "")
        token = auth_header.removeprefix("Bearer ").strip()
        record = validate_api_token(token)
        if record is None:
            return jsonify({"error": "Invalid or missing API token"}), 401
        request.api_record = record
        return f(*args, **kwargs)
    return decorated
