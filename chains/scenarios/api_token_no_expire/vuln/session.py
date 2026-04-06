"""API token management -- VULNERABLE variant.

API tokens are issued at account registration and stored without any expiry
timestamp.  Tokens are never rotated or invalidated unless the user explicitly
revokes them.  A token leaked via a past breach, log file, or shoulder surfing
remains valid indefinitely, giving an attacker permanent access (CWE-384).

Chain: token issued at registration -> leaked (breach / log) -> replayed
       months later -> full API access (no expiry check)
Individual findings: API token without expiry (medium)
Chain finding: leaked non-expiring token enables indefinite API access (high)
"""
import secrets
import time
import functools
from flask import request, jsonify

# {token: {user_id, role, issued_at}}  -- no expires_at field
_api_tokens = {}


def issue_api_token(user_id, role):
    """Generate a new API token for the user.

    VULNERABLE: no expiry timestamp is recorded, so the token lives forever
    until explicitly revoked.
    """
    token = secrets.token_hex(32)
    _api_tokens[token] = {
        "user_id": user_id,
        "role": role,
        "issued_at": time.time(),
    }
    return token


def revoke_api_token(token):
    """Remove an API token from the store."""
    _api_tokens.pop(token, None)


def validate_api_token(token):
    """Return token record if the token is valid.

    VULNERABLE: only checks existence -- never compares against an expiry.
    """
    if not token:
        return None
    return _api_tokens.get(token)


def require_api_auth(f):
    """Decorator enforcing a valid API token from Authorization header."""
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
