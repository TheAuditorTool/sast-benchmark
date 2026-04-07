"""Admin endpoint guarded by a static token.

This file is IDENTICAL between vuln/ and safe/ variants.

The swagger.json reveals the existence of this endpoint and the header
name it requires, dramatically reducing the brute-force search space.

CWE-200: Swagger exposure of admin endpoint enables targeted token brute-force.
Chain: GET /swagger.json reveals /admin/create-user -> attacker targets token -> admin access gained
"""
import functools
from flask import request, jsonify
from config import app, ADMIN_TOKEN, USERS
from debug import swagger  # noqa: F401 - registers /swagger.json route


def _require_admin_token(f):
    """Require X-Admin-Token header."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        if request.headers.get("X-Admin-Token") != ADMIN_TOKEN:
            return jsonify({"error": "Forbidden"}), 403
        return f(*args, **kwargs)
    return decorated


@app.route("/admin/create-user", methods=["POST"])
@_require_admin_token
def create_user():
    """Create a new user (admin only)."""
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    if not username:
        return jsonify({"error": "username required"}), 400
    USERS[username] = {"password": data.get("password", ""), "role": data.get("role", "user")}
    return jsonify({"status": "created"}), 201


if __name__ == "__main__":
    app.run(port=5000)
