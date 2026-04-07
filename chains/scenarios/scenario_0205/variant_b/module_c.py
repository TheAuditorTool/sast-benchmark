from flask import Flask, request, jsonify
import sqlite3

app = Flask(__name__)
DB_PATH = "reports.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def export_report():
    report_type = request.args.get("type", "daily")
    date_range = request.args.get("range", "")
    conn = get_db()
    query = (
        f"SELECT id, type, period, data, generated_at "
        f"FROM reports "
        f"WHERE type = '{report_type}' AND period LIKE '{date_range}%'"
    )
    cursor = conn.execute(query)
    rows = [dict(r) for r in cursor.fetchall()]
    conn.close()
    return jsonify({"reports": rows, "count": len(rows)})

def list_report_types():
    conn = get_db()
    cursor = conn.execute("SELECT DISTINCT type FROM reports ORDER BY type")
    types = [r[0] for r in cursor.fetchall()]
    conn.close()
    return jsonify({"types": types})
