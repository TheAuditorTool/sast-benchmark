from flask import Flask, request, jsonify
import sqlite3

app = Flask(__name__)
DB_PATH = "internal.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

# vuln-code-snippet start ChainScenario0071B
def run_query():
    table = request.args.get("table", "events")
    filter_val = request.args.get("filter", "")
    conn = get_db()
    query = f"SELECT * FROM {table} WHERE description LIKE '%{filter_val}%' LIMIT 100"
    cursor = conn.execute(query)  # vuln-code-snippet target-line ChainScenario0071B
    rows = [dict(r) for r in cursor.fetchall()]
    conn.close()
    return jsonify({"rows": rows, "count": len(rows)})
# vuln-code-snippet end ChainScenario0071B

def list_tables():
    conn = get_db()
    cursor = conn.execute("SELECT name FROM sqlite_master WHERE type='table'")
    tables = [r[0] for r in cursor.fetchall()]
    conn.close()
    return jsonify({"tables": tables})
