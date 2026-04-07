"""Admin dashboard query routes with SQL injection.

This file is IDENTICAL between vuln/ and safe/ variants.
The SQL injection exists in both -- what differs is whether
middleware.py rate-limits login attempts (see middleware.py).

Chain: brute-force login (no rate limit) -> valid session -> SQLi on /admin/query
"""
from flask import Flask, request, jsonify
import sqlite3

app = Flask(__name__)
DB_PATH = "admin.db"


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


# vuln-code-snippet start chain_rate_limit_sqli_vuln
def admin_query():
    """Run an administrative data query. Requires admin session."""
    table = request.args.get("table", "audit_log")
    filter_val = request.args.get("filter", "")
    conn = get_db()
    query = f"SELECT * FROM {table} WHERE details LIKE '%{filter_val}%' ORDER BY id DESC LIMIT 200"
    cursor = conn.execute(query)  # vuln-code-snippet vuln-line chain_rate_limit_sqli_vuln
    rows = [dict(r) for r in cursor.fetchall()]
    conn.close()
    return jsonify({"rows": rows, "count": len(rows)})
# vuln-code-snippet end chain_rate_limit_sqli_vuln


def audit_summary():
    """Return a count of audit events per action type."""
    conn = get_db()
    cursor = conn.execute(
        "SELECT action, COUNT(*) as count FROM audit_log GROUP BY action ORDER BY count DESC"
    )
    rows = [dict(r) for r in cursor.fetchall()]
    conn.close()
    return jsonify({"summary": rows})
