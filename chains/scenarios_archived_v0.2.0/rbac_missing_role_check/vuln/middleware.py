"""Authentication middleware -- VULNERABLE variant.

The middleware verifies that the request carries a valid Bearer token
(i.e., the user is authenticated), but does not check whether the user
has the 'admin' role. Any authenticated user -- including low-privilege
read-only accounts -- can call the admin analytics endpoint.

Chain: authenticated non-admin user -> role not checked -> SQLi on /analytics/query
Individual findings: missing RBAC (high) + SQLi (medium)
Chain finding: authenticated non-admin reaching SQLi (critical)
"""
import hmac
import hashlib
import base64
import json
import functools
from flask import request, jsonify

SECRET_KEY = "analytics-service-secret"


def _verify_token(token):
    """Verify a signed token and return the payload dict or None."""
    if not token:
        return None
    parts = token.split(".")
    if len(parts) != 3:
        return None
    try:
        payload = json.loads(base64.urlsafe_b64decode(parts[1] + "=="))
    except Exception:
        return None
    signing_input = parts[0] + "." + parts[1]
    expected = hmac.new(SECRET_KEY.encode(), signing_input.encode(), hashlib.sha256).hexdigest()
    if not hmac.compare_digest(expected, parts[2]):
        return None
    return payload


def require_auth(f):
    """Decorator that verifies authentication only.

    VULNERABLE: confirms the token is valid but does not enforce that
    request.user_role == 'admin'. Any authenticated user can reach the endpoint.
    """
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        auth_header = request.headers.get("Authorization", "")
        if not auth_header.startswith("Bearer "):
            return jsonify({"error": "Authentication required"}), 401
        token = auth_header[7:]
        payload = _verify_token(token)
        if payload is None:
            return jsonify({"error": "Invalid or expired token"}), 401
        request.user_id = payload.get("sub", "")
        request.user_role = payload.get("role", "viewer")
        # VULNERABLE: returns here without checking user_role == 'admin'
        return f(*args, **kwargs)
    return decorated
