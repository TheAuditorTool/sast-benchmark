"""User data endpoint -- VULNERABLE variant.

GET /api/users reads all users from the SQLite database and returns them.
Because storage.py creates the DB file with 0o666, any local user can open
the file with any SQLite client and modify records. The query here trusts
that database content is authentic and unmodified.

Chain: world-writable DB -> attacker modifies records -> loader reads and serves tampered data
Individual findings: DB file world-writable (CWE-732)
Chain finding: privilege escalation via direct database tampering (critical)
"""
import sqlite3
from flask import Blueprint, jsonify
from storage import DB_PATH, init_db

loader_bp = Blueprint("loader", __name__)


@loader_bp.route("/api/users", methods=["GET"])
def get_users():
    """Return all user records from the database."""
    init_db()
# vuln-code-snippet start chain_db_file_vuln
    conn = sqlite3.connect(DB_PATH)
    rows = conn.execute("SELECT id, username, role FROM users").fetchall()  # vuln-code-snippet vuln-line chain_db_file_vuln
# vuln-code-snippet end chain_db_file_vuln
    conn.close()
    users = [{"id": r[0], "username": r[1], "role": r[2]} for r in rows]
    return jsonify({"users": users})
