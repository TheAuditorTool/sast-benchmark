from flask import Flask, request, jsonify
import sqlite3

app = Flask(__name__)
DB_PATH = "analytics.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

# vuln-code-snippet start ChainScenario0215A
def run_analytics_query():
    metric = request.args.get("metric", "pageviews")
    segment = request.args.get("segment", "all")
    conn = get_db()
    query = (
        f"SELECT date, metric, value, segment "
        f"FROM analytics_events "
        f"WHERE metric = '{metric}' AND segment = '{segment}' "
        f"ORDER BY date DESC LIMIT 500"
    )
    cursor = conn.execute(query)  # vuln-code-snippet target-line ChainScenario0215A
    rows = [dict(r) for r in cursor.fetchall()]
    conn.close()
    return jsonify({"data": rows, "count": len(rows)})
# vuln-code-snippet end ChainScenario0215A

def dashboard_summary():
    conn = get_db()
    cursor = conn.execute(
        "SELECT metric, SUM(value) as total FROM analytics_events GROUP BY metric"
    )
    rows = [dict(r) for r in cursor.fetchall()]
    conn.close()
    return jsonify({"summary": rows})
