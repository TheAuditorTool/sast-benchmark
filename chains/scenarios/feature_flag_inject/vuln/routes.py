"""User preferences routes.

This file is IDENTICAL between vuln/ and safe/ variants.

Chain:
  1. PUT /preferences passes the full body to deserialize_preferences.
  2. The returned dict may contain 'feature_flags' listing admin-only flags.
  3. The preferences record is updated, granting the user elevated features.

CWE-915: Mass assignment of feature_flags enables access to admin-only functionality.
"""
import functools
from flask import request, jsonify
from models import app, PREFERENCES
from serializers import deserialize_preferences


def _require_auth(f):
    """Require X-User-Id header."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        if request.headers.get("X-User-Id") not in PREFERENCES:
            return jsonify({"error": "Authentication required"}), 401
        return f(*args, **kwargs)
    return decorated


@app.route("/preferences", methods=["PUT"])
@_require_auth
def update_preferences():
    """Replace the caller's preferences."""
    user_id = request.headers.get("X-User-Id")
    data = request.get_json(force=True) or {}
    updates = deserialize_preferences(data)
    PREFERENCES[user_id].update(updates)
    return jsonify({"status": "updated", "preferences": PREFERENCES[user_id]})


if __name__ == "__main__":
    app.run(port=5000)
