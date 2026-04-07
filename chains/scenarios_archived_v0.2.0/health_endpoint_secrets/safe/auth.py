"""Database-backed privileged action.

This file is IDENTICAL between vuln/ and safe/ variants.

Uses the DATABASE_URL from config.  Because debug.py leaks that URL, an
attacker can connect to the database directly without going through the API.

CWE-200: Leaked DB connection string enables direct database access.
Chain: GET /health returns DATABASE_URL with password -> attacker connects to DB directly
"""
import functools
from flask import request, jsonify
from debug import app
from config import JWT_SECRET


def _require_auth(f):
    """Require X-JWT-Token header matching JWT_SECRET."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        if request.headers.get("X-JWT-Token") != JWT_SECRET:
            return jsonify({"error": "Authentication required"}), 401
        return f(*args, **kwargs)
    return decorated


@app.route("/admin/users")
@_require_auth
def list_users():
    """List all users. Requires JWT auth."""
    return jsonify({"users": ["alice", "bob", "admin"]})


if __name__ == "__main__":
    app.run(port=5000)
