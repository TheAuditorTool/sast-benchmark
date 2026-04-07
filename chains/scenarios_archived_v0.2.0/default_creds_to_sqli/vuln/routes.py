"""User management routes with SQL injection.

This file is IDENTICAL between vuln/ and safe/ variants.
The SQL injection exists in both -- what differs is whether
auth.py allows login with default credentials (see auth.py).

Chain: default creds (admin/admin) -> login -> valid session -> SQLi on /admin/users/search
"""
from flask import Flask, request, jsonify
import sqlite3
import config

app = Flask(__name__)


def get_db():
    conn = sqlite3.connect(config.DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


# vuln-code-snippet start chain_default_creds_sqli_vuln
def search_users():
    """Search users by email domain. Admin-only endpoint."""
    domain = request.args.get("domain", "")
    conn = get_db()
    query = f"SELECT id, username, email, created_at FROM users WHERE email LIKE '%{domain}'"
    cursor = conn.execute(query)  # vuln-code-snippet vuln-line chain_default_creds_sqli_vuln
    rows = [dict(r) for r in cursor.fetchall()]
    conn.close()
    return jsonify({"users": rows})
# vuln-code-snippet end chain_default_creds_sqli_vuln


def delete_user():
    """Delete a user by ID. Admin-only endpoint."""
    data = request.get_json(silent=True) or {}
    user_id = data.get("user_id")
    if not user_id:
        return jsonify({"error": "user_id required"}), 400
    conn = get_db()
    conn.execute("DELETE FROM users WHERE id = ?", (user_id,))
    conn.commit()
    conn.close()
    return jsonify({"status": "deleted", "user_id": user_id})
