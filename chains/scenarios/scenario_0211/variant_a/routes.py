from flask import Flask, request, jsonify
import sqlite3

app = Flask(__name__)
DB_PATH = "admin.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

# vuln-code-snippet start ChainScenario0211A
def admin_query():
    table = request.args.get("table", "audit_log")
    filter_val = request.args.get("filter", "")
    conn = get_db()
    query = f"SELECT * FROM {table} WHERE details LIKE '%{filter_val}%' ORDER BY id DESC LIMIT 200"
    cursor = conn.execute(query)  # vuln-code-snippet target-line ChainScenario0211A
    rows = [dict(r) for r in cursor.fetchall()]
    conn.close()
    return jsonify({"rows": rows, "count": len(rows)})
# vuln-code-snippet end ChainScenario0211A

def audit_summary():
    conn = get_db()
    cursor = conn.execute(
        "SELECT action, COUNT(*) as count FROM audit_log GROUP BY action ORDER BY count DESC"
    )
    rows = [dict(r) for r in cursor.fetchall()]
    conn.close()
    return jsonify({"summary": rows})
