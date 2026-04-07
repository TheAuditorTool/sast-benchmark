"""Data export routes with SQL injection.

This file is IDENTICAL between vuln/ and safe/ variants.
The SQL injection exists in both -- what differs is whether
middleware.py reads the API key from the URL (loggable) or a header (see middleware.py).

Chain: API key leaked from URL log -> replayed in Authorization header -> SQLi on /export
"""
from flask import Flask, request, jsonify
import sqlite3

app = Flask(__name__)
DB_PATH = "warehouse.db"


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


# vuln-code-snippet start chain_api_key_leak_sqli_vuln
def export_records():
    """Export records matching a customer and date filter."""
    customer = request.args.get("customer", "")
    start_date = request.args.get("start", "")
    end_date = request.args.get("end", "")
    conn = get_db()
    query = (
        f"SELECT id, customer, amount, status, created_at "
        f"FROM orders "
        f"WHERE customer = '{customer}' "
        f"AND created_at BETWEEN '{start_date}' AND '{end_date}'"
    )
    cursor = conn.execute(query)  # vuln-code-snippet vuln-line chain_api_key_leak_sqli_vuln
    rows = [dict(r) for r in cursor.fetchall()]
    conn.close()
    return jsonify({"records": rows, "count": len(rows)})
# vuln-code-snippet end chain_api_key_leak_sqli_vuln


def schema_info():
    """Return available table names for the data warehouse."""
    conn = get_db()
    cursor = conn.execute("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
    tables = [r[0] for r in cursor.fetchall()]
    conn.close()
    return jsonify({"tables": tables})
