"""Internal data query routes with SQL injection.

This file is IDENTICAL between vuln/ and safe/ variants.
The SQL injection exists in both -- what differs is whether
middleware.py can be bypassed by spoofing X-Forwarded-For (see middleware.py).

Chain: spoofed X-Forwarded-For header -> IP check bypassed -> SQLi on /internal/query
"""
from flask import Flask, request, jsonify
import sqlite3

app = Flask(__name__)
DB_PATH = "internal.db"


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


# vuln-code-snippet start chain_ip_allowlist_sqli_safe
def run_query():
    """Execute a filtered data query. Internal network only."""
    table = request.args.get("table", "events")
    filter_val = request.args.get("filter", "")
    conn = get_db()
    query = f"SELECT * FROM {table} WHERE description LIKE '%{filter_val}%' LIMIT 100"
    cursor = conn.execute(query)  # vuln-code-snippet safe-line chain_ip_allowlist_sqli_safe
    rows = [dict(r) for r in cursor.fetchall()]
    conn.close()
    return jsonify({"rows": rows, "count": len(rows)})
# vuln-code-snippet end chain_ip_allowlist_sqli_safe


def list_tables():
    """List available tables. Internal network only."""
    conn = get_db()
    cursor = conn.execute("SELECT name FROM sqlite_master WHERE type='table'")
    tables = [r[0] for r in cursor.fetchall()]
    conn.close()
    return jsonify({"tables": tables})
