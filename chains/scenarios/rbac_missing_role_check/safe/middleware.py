"""Authentication middleware -- SAFE variant.

The middleware verifies both authentication (valid token) and authorization
(role == 'admin'). A valid token for a viewer or analyst account is
rejected with 403 before the request reaches the analytics endpoint.

Chain: authenticated non-admin user -> role check fails -> 403 (chain broken)
Individual findings: SQLi still present in routes.py (medium)
Chain finding: none -- non-admin role is enforced at the middleware layer
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
    """Decorator that verifies authentication AND enforces admin role.

    SAFE: checks that the token's role claim is 'admin' before proceeding.
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
        # SAFE: enforce admin role for analytics endpoints
        if request.user_role != "admin":
            return jsonify({"error": "Admin role required"}), 403
        return f(*args, **kwargs)
    return decorated
