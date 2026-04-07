"""Report export routes with SQL injection.

This file is IDENTICAL between vuln/ and safe/ variants.
The SQL injection exists in both -- what differs is whether
middleware.py allows Basic auth over plain HTTP (see middleware.py).

Chain: credentials sniffed over HTTP -> replayed -> SQLi on /reports/export
"""
from flask import Flask, request, jsonify
import sqlite3

app = Flask(__name__)
DB_PATH = "reports.db"


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


# vuln-code-snippet start chain_basic_auth_sqli_vuln
def export_report():
    """Export report data for a given date range and report type."""
    report_type = request.args.get("type", "daily")
    date_range = request.args.get("range", "")
    conn = get_db()
    query = (
        f"SELECT id, type, period, data, generated_at "
        f"FROM reports "
        f"WHERE type = '{report_type}' AND period LIKE '{date_range}%'"
    )
    cursor = conn.execute(query)  # vuln-code-snippet vuln-line chain_basic_auth_sqli_vuln
    rows = [dict(r) for r in cursor.fetchall()]
    conn.close()
    return jsonify({"reports": rows, "count": len(rows)})
# vuln-code-snippet end chain_basic_auth_sqli_vuln


def list_report_types():
    """List available report types."""
    conn = get_db()
    cursor = conn.execute("SELECT DISTINCT type FROM reports ORDER BY type")
    types = [r[0] for r in cursor.fetchall()]
    conn.close()
    return jsonify({"types": types})
