"""JWT-authenticated endpoint.

This file is IDENTICAL between vuln/ and safe/ variants.

Validates JWTs using JWT_SECRET from config.  Because debug.py's
/actuator/env endpoint leaks JWT_SECRET, an attacker can forge any JWT.

CWE-200: Actuator env leaks JWT_SECRET enabling JWT forgery and account takeover.
Chain: GET /actuator/env returns JWT_SECRET -> attacker forges admin JWT -> privileged action executed
"""
import functools
import base64
import hmac
import hashlib
import json
from flask import request, jsonify
from debug import app
from config import JWT_SECRET


def _decode_jwt(token):
    """Decode and verify a minimal HS256 JWT (header.payload.sig, base64url)."""
    try:
        parts = token.split(".")
        if len(parts) != 3:
            return None
        payload_b64 = parts[1] + "=="
        payload = json.loads(base64.urlsafe_b64decode(payload_b64))
        signing_input = (parts[0] + "." + parts[1]).encode()
        expected_sig = base64.urlsafe_b64encode(
            hmac.new(JWT_SECRET.encode(), signing_input, hashlib.sha256).digest()
        ).rstrip(b"=").decode()
        if not hmac.compare_digest(parts[2], expected_sig):
            return None
        return payload
    except Exception:
        return None


def jwt_required(f):
    """Require a valid Authorization: Bearer <jwt> header."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        auth = request.headers.get("Authorization", "")
        if not auth.startswith("Bearer "):
            return jsonify({"error": "Authentication required"}), 401
        payload = _decode_jwt(auth[7:])
        if payload is None:
            return jsonify({"error": "Invalid token"}), 401
        request.jwt_payload = payload
        return f(*args, **kwargs)
    return decorated


@app.route("/api/admin/settings", methods=["POST"])
@jwt_required
def update_admin_settings():
    """Update admin settings. Requires admin role in JWT."""
    if request.jwt_payload.get("role") != "admin":
        return jsonify({"error": "Forbidden"}), 403
    data = request.get_json(force=True) or {}
    return jsonify({"status": "settings updated", "applied": data})


if __name__ == "__main__":
    app.run(port=5000)
