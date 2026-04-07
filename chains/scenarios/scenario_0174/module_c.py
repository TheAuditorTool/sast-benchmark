from flask import Flask, request, jsonify
import sqlite3

app = Flask(__name__)
DB_PATH = "warehouse.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def export_records():
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
    cursor = conn.execute(query)
    rows = [dict(r) for r in cursor.fetchall()]
    conn.close()
    return jsonify({"records": rows, "count": len(rows)})

def schema_info():
    conn = get_db()
    cursor = conn.execute("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
    tables = [r[0] for r in cursor.fetchall()]
    conn.close()
    return jsonify({"tables": tables})
