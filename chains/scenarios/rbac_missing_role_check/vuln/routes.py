"""Admin analytics query routes with SQL injection.

This file is IDENTICAL between vuln/ and safe/ variants.
The SQL injection exists in both -- what differs is whether
middleware.py checks the user's role before granting access (see middleware.py).

Chain: non-admin user token -> role not checked -> SQLi on /analytics/query
"""
from flask import Flask, request, jsonify
import sqlite3

app = Flask(__name__)
DB_PATH = "analytics.db"


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


# vuln-code-snippet start chain_rbac_missing_sqli_vuln
def run_analytics_query():
    """Run a custom analytics query. Admin-only endpoint."""
    metric = request.args.get("metric", "pageviews")
    segment = request.args.get("segment", "all")
    conn = get_db()
    query = (
        f"SELECT date, metric, value, segment "
        f"FROM analytics_events "
        f"WHERE metric = '{metric}' AND segment = '{segment}' "
        f"ORDER BY date DESC LIMIT 500"
    )
    cursor = conn.execute(query)  # vuln-code-snippet vuln-line chain_rbac_missing_sqli_vuln
    rows = [dict(r) for r in cursor.fetchall()]
    conn.close()
    return jsonify({"data": rows, "count": len(rows)})
# vuln-code-snippet end chain_rbac_missing_sqli_vuln


def dashboard_summary():
    """Return pre-computed dashboard stats. Authenticated users."""
    conn = get_db()
    cursor = conn.execute(
        "SELECT metric, SUM(value) as total FROM analytics_events GROUP BY metric"
    )
    rows = [dict(r) for r in cursor.fetchall()]
    conn.close()
    return jsonify({"summary": rows})
