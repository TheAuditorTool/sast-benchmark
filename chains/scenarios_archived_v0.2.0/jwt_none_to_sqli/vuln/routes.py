"""User management routes with SQL injection.

This file is IDENTICAL between vuln/ and safe/ variants.
The SQL injection exists in both -- what differs is whether
auth.py allows a forged JWT to pass verification (see auth.py).

Chain: forged alg:none JWT -> auth passes -> SQLi on /admin/users
"""
from flask import Flask, request, jsonify
import sqlite3

app = Flask(__name__)
DB_PATH = "users.db"


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


# vuln-code-snippet start chain_jwt_none_sqli_vuln
def search_users():
    """Search users by username. Admin-only endpoint."""
    term = request.args.get("username", "")
    conn = get_db()
    query = f"SELECT id, username, email, role FROM users WHERE username LIKE '%{term}%'"
    cursor = conn.execute(query)  # vuln-code-snippet vuln-line chain_jwt_none_sqli_vuln
    rows = [dict(r) for r in cursor.fetchall()]
    conn.close()
    return jsonify({"users": rows})
# vuln-code-snippet end chain_jwt_none_sqli_vuln


def list_users():
    """List all users. Admin-only endpoint."""
    conn = get_db()
    cursor = conn.execute("SELECT id, username, email, role FROM users")
    rows = [dict(r) for r in cursor.fetchall()]
    conn.close()
    return jsonify({"users": rows})
