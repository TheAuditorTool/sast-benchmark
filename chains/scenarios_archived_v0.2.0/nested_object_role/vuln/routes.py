"""Nested profile settings routes.

This file is IDENTICAL between vuln/ and safe/ variants.

Chain:
  1. PATCH /profile passes the body to deserialize_profile_update.
  2. The deserializer strips top-level 'role' but merges nested 'settings' wholesale.
  3. An attacker supplies {'settings': {'role': 'admin'}} to bypass the filter.

CWE-915: Nested object mass assignment bypasses top-level field filter.
"""
import functools
from flask import request, jsonify
from models import app, USERS
from serializers import deserialize_profile_update


def _require_auth(f):
    """Require X-User-Id header."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        if request.headers.get("X-User-Id") not in USERS:
            return jsonify({"error": "Authentication required"}), 401
        return f(*args, **kwargs)
    return decorated


@app.route("/profile", methods=["PATCH"])
@_require_auth
def update_profile():
    """Update the caller's profile."""
    user_id = request.headers.get("X-User-Id")
    data = request.get_json(force=True) or {}
    existing_settings = dict(USERS[user_id].get("settings", {}))
    updates = deserialize_profile_update(data, existing_settings)
    USERS[user_id].update(updates)
    return jsonify({"status": "updated", "user": USERS[user_id]})


if __name__ == "__main__":
    app.run(port=5000)
