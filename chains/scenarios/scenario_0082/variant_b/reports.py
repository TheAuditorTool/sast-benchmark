import sqlite3
from flask import Blueprint, request, jsonify

reports_bp = Blueprint("reports", __name__)
DB_PATH = "reports.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def init_db():
    conn = get_db()
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sales "
        "(id INTEGER PRIMARY KEY AUTOINCREMENT, region TEXT, amount REAL)"
    )
    conn.commit()
    conn.close()

# vuln-code-snippet start ChainScenario0082B
@reports_bp.route("/api/report", methods=["POST"])
def generate_report():
    region = request.form.get("region", "all")
    conn = get_db()
    cursor = conn.execute(
        "SELECT id, region, amount FROM sales WHERE region = ?", (region,)
    )  # vuln-code-snippet target-line ChainScenario0082B
    rows = cursor.fetchall()
    conn.close()
    return jsonify({"results": [dict(r) for r in rows]})
# vuln-code-snippet end ChainScenario0082B

init_db()
