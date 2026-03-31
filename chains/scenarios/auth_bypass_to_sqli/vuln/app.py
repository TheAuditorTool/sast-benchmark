"""Admin search endpoint with SQL injection.

This file is IDENTICAL between vuln/ and safe/ variants.
The vulnerability exists in both -- what differs is whether
an unauthenticated user can reach it (see routes.py).
"""
from flask import Flask, request, jsonify
import sqlite3

app = Flask(__name__)
DB_PATH = "app.db"


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


# vuln-code-snippet start chain_auth_sqli_vuln
def admin_search_users():
    """Search users by name. Intended for admin use only."""
    search = request.args.get("q", "")
    conn = get_db()
    # String formatting in SQL query -- injectable
    query = f"SELECT id, username, email FROM users WHERE username LIKE '%{search}%'"
    cursor = conn.execute(query)  # vuln-code-snippet vuln-line chain_auth_sqli_vuln
    results = [dict(row) for row in cursor.fetchall()]
    conn.close()
    return jsonify(results)
# vuln-code-snippet end chain_auth_sqli_vuln


def admin_dashboard():
    """Render admin dashboard. Requires authentication."""
    return jsonify({"status": "ok", "page": "admin_dashboard"})
