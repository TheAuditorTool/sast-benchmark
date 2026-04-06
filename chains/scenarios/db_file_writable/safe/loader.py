"""User data endpoint -- SAFE variant.

GET /api/users reads all users from the SQLite database. Because storage.py
creates the DB file with 0o600, no other process can open the file directly.
The records returned here are guaranteed to be unmodified by external actors.

Chain broken: DB file is owner-only -> no external SQLite access -> loader reads authentic records
"""
import sqlite3
from flask import Blueprint, jsonify
from storage import DB_PATH, init_db

loader_bp = Blueprint("loader", __name__)


@loader_bp.route("/api/users", methods=["GET"])
def get_users():
    """Return all user records from the database."""
    init_db()
# vuln-code-snippet start chain_db_file_safe
    conn = sqlite3.connect(DB_PATH)
    rows = conn.execute("SELECT id, username, role FROM users").fetchall()  # vuln-code-snippet safe-line chain_db_file_safe
# vuln-code-snippet end chain_db_file_safe
    conn.close()
    users = [{"id": r[0], "username": r[1], "role": r[2]} for r in rows]
    return jsonify({"users": users})
