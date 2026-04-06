"""API key routes.

This file is IDENTICAL between vuln/ and safe/ variants.

Chain:
  1. POST /api-keys passes the full JSON body to deserialize_key_create.
  2. The returned spec includes whatever scopes the caller specified.
  3. The key is stored and returned; privileged scopes are now usable.

CWE-915: Mass assignment of API key scopes enables privilege escalation.
"""
import functools
import secrets
from flask import request, jsonify
from models import app, API_KEYS
from serializers import deserialize_key_create

_counter = [1]


def _require_auth(f):
    """Require X-User-Id header."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        if not request.headers.get("X-User-Id"):
            return jsonify({"error": "Authentication required"}), 401
        return f(*args, **kwargs)
    return decorated


@app.route("/api-keys", methods=["POST"])
@_require_auth
def create_api_key():
    """Create a new API key for the authenticated user."""
    owner_id = request.headers.get("X-User-Id")
    data = request.get_json(force=True) or {}
    spec = deserialize_key_create(data)
    key_id = str(_counter[0])
    _counter[0] += 1
    API_KEYS[key_id] = {
        "owner_id": owner_id,
        "name": spec["name"],
        "scopes": spec["scopes"],
        "token": secrets.token_hex(16),
        "active": True,
    }
    return jsonify({"id": key_id, "scopes": spec["scopes"]}), 201


if __name__ == "__main__":
    app.run(port=5000)
