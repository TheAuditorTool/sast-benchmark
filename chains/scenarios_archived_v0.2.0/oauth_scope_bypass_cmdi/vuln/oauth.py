"""OAuth token validation module -- VULNERABLE variant.

The token validator verifies that a Bearer token exists and is structurally
valid (signed correctly), but does not check the 'scope' claim. Any valid
OAuth token -- including tokens issued to low-privilege clients -- can reach
the deploy endpoint, regardless of what scopes they were granted.

Chain: low-scope OAuth token -> scope not checked -> CMDi on /deploy/run
Individual findings: missing scope check (high) + CMDi (high)
Chain finding: unauthorized CMDi with any valid OAuth token (critical)
"""
import base64
import json
import hmac
import hashlib
import functools
from flask import request, jsonify

SECRET_KEY = "oauth-server-signing-secret"


def _b64url_decode(s):
    """Decode base64url without padding."""
    s += "=" * (-len(s) % 4)
    return base64.urlsafe_b64decode(s)


def validate_token(token):
    """Validate an OAuth Bearer token. Returns payload dict or None.

    VULNERABLE: verifies signature but does not check the 'scope' claim.
    A token with scope 'read:repo' can access a write endpoint.
    """
    if not token:
        return None
    parts = token.split(".")
    if len(parts) != 3:
        return None
    try:
        payload = json.loads(_b64url_decode(parts[1]))
    except Exception:
        return None

    signing_input = parts[0] + "." + parts[1]
    expected_sig = hmac.new(
        SECRET_KEY.encode(), signing_input.encode(), hashlib.sha256
    ).digest()
    expected_b64 = base64.urlsafe_b64encode(expected_sig).rstrip(b"=").decode()
    if not hmac.compare_digest(expected_b64, parts[2]):
        return None
    # VULNERABLE: returns payload without verifying scope
    return payload


def require_deploy_scope(f):
    """Decorator that validates an OAuth token for the deploy endpoint."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        auth_header = request.headers.get("Authorization", "")
        if not auth_header.startswith("Bearer "):
            return jsonify({"error": "Missing Bearer token"}), 401
        token = auth_header[7:]
        payload = validate_token(token)
        if payload is None:
            return jsonify({"error": "Invalid token"}), 401
        request.token_payload = payload
        return f(*args, **kwargs)
    return decorated
