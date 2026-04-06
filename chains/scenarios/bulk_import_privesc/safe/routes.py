"""Bulk import route.

This file is IDENTICAL between vuln/ and safe/ variants.

Chain:
  1. POST /import accepts a CSV body.
  2. parse_import_csv converts each row to a dict, including the 'role' column.
  3. Each user dict is stored, potentially creating admin accounts.

CWE-915: Mass assignment via CSV role column enables bulk privilege escalation.
"""
import functools
from flask import request, jsonify
from models import app, USERS, DEFAULT_ROLE
from serializers import parse_import_csv


def _require_admin(f):
    """Require X-User-Id header with role admin."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        caller_id = request.headers.get("X-User-Id", "")
        if USERS.get(caller_id, {}).get("role") != "admin":
            return jsonify({"error": "Admin required"}), 403
        return f(*args, **kwargs)
    return decorated


@app.route("/import", methods=["POST"])
@_require_admin
def bulk_import():
    """Import users from a CSV body."""
    raw = request.get_data(as_text=True)
    rows = parse_import_csv(raw)
    created = []
    for row in rows:
        username = row.get("username", "").strip()
        if not username:
            continue
        USERS[username] = {
            "email": row.get("email", ""),
            "role": row.get("role", DEFAULT_ROLE),
        }
        created.append(username)
    return jsonify({"created": created}), 201


if __name__ == "__main__":
    app.run(port=5000)
