"""API token authentication middleware -- VULNERABLE variant.

The token comparison uses string equality instead of constant-time
comparison. An attacker can exploit timing differences to brute-force
the token character by character, bypassing authentication.

Chain: timing attack on token comparison -> auth bypass -> command injection
"""
from flask import request, jsonify
import functools

VALID_TOKEN = "admin-bearer-token-2026-production"


def require_auth(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        token = request.headers.get("X-API-Token", "")
        if not token:
            return jsonify({"error": "Missing token"}), 401
        # VULNERABLE: string equality comparison is timing-attackable
        if token != VALID_TOKEN:
            return jsonify({"error": "Invalid token"}), 401
        return f(*args, **kwargs)
    return decorated
