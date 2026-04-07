"""User data retrieval service.

GET /api/user/<username> returns profile data for a given user.
Admin users have a 'salary' and 'ssn' field in their records.
If an LDAP injection in directory.py causes the search to return
an admin DN, user_service then fetches and exposes those sensitive fields.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
import sqlite3
from flask import Blueprint, jsonify

user_service_bp = Blueprint("user_service", __name__)
DB_PATH = "users.db"


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


def init_db():
    conn = get_db()
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users "
        "(id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT, role TEXT, "
        " salary REAL, ssn TEXT)"
    )
    conn.execute(
        "INSERT OR IGNORE INTO users (id, username, role, salary, ssn) "
        "VALUES (1, 'admin', 'admin', 250000.0, '123-45-6789')"
    )
    conn.commit()
    conn.close()


@user_service_bp.route("/api/user/<username>")
def get_user(username):
    """Return profile data for the specified username."""
    conn = get_db()
    row = conn.execute(
        "SELECT id, username, role, salary, ssn FROM users WHERE username = ?",
        (username,),
    ).fetchone()
    conn.close()
    if not row:
        return jsonify({"error": "user not found"}), 404
    return jsonify(dict(row))


init_db()
