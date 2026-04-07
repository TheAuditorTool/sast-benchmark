"""JWT authentication middleware -- SAFE variant.

The token comparison uses hmac.compare_digest for constant-time
comparison, preventing timing attacks. The chain is broken because
the attacker cannot brute-force the token.
"""
import hmac
from flask import request, jsonify
import functools

VALID_TOKEN = "admin-bearer-token-2026-production"


def require_auth(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        token = request.headers.get("X-API-Token", "")
        if not token:
            return jsonify({"error": "Missing token"}), 401
        # SAFE: constant-time comparison prevents timing attacks
        if not hmac.compare_digest(token, VALID_TOKEN):
            return jsonify({"error": "Invalid token"}), 401
        return f(*args, **kwargs)
    return decorated
