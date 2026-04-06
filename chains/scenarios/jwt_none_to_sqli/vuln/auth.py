"""JWT verification module -- VULNERABLE variant.

The verifier accepts the 'none' algorithm declared in the JWT header,
which means no signature is checked. An attacker can forge a token with
arbitrary claims (e.g., any user_id) by base64-encoding a crafted header
and payload and omitting the signature.

Chain: alg:none forged JWT -> auth bypass -> SQLi on /admin/users
Individual findings: JWT alg:none (high) + SQLi (medium)
Chain finding: unauthenticated SQLi (critical)
"""
import base64
import json
import hmac
import hashlib
from flask import request, jsonify
import functools

SECRET_KEY = "prod-jwt-secret-do-not-share"


def _b64url_decode(s):
    """Decode base64url without padding."""
    s += "=" * (-len(s) % 4)
    return base64.urlsafe_b64decode(s)


def verify_jwt(token):
    """Verify a JWT token. Returns decoded payload dict or None.

    VULNERABLE: accepts algorithm 'none' from the token header, skipping
    the signature verification step entirely.
    """
    if not token:
        return None
    parts = token.split(".")
    if len(parts) != 3:
        return None
    try:
        header = json.loads(_b64url_decode(parts[0]))
        payload = json.loads(_b64url_decode(parts[1]))
    except Exception:
        return None

    alg = header.get("alg", "HS256")
    if alg == "none":
        # VULNERABLE: skips signature check when attacker declares alg:none
        return payload

    signing_input = parts[0] + "." + parts[1]
    expected_sig = hmac.new(
        SECRET_KEY.encode(), signing_input.encode(), hashlib.sha256
    ).digest()
    expected_b64 = base64.urlsafe_b64encode(expected_sig).rstrip(b"=").decode()
    if not hmac.compare_digest(expected_b64, parts[2]):
        return None
    return payload


def require_auth(f):
    """Decorator that verifies a JWT Bearer token."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        auth_header = request.headers.get("Authorization", "")
        if not auth_header.startswith("Bearer "):
            return jsonify({"error": "Missing authorization token"}), 401
        token = auth_header[7:]
        payload = verify_jwt(token)
        if payload is None:
            return jsonify({"error": "Invalid or expired token"}), 401
        request.jwt_payload = payload
        return f(*args, **kwargs)
    return decorated
