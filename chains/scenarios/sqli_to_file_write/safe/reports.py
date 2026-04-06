"""Report generation endpoint with SQL injection -- SAFE variant.

POST /api/report uses a parameterized query with a ? placeholder so
the region filter cannot be used to inject SQL. The INTO OUTFILE
technique is therefore blocked and no webshell can be written.

Chain broken: parameterized query prevents injection -> no file write
"""
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


# vuln-code-snippet start chain_sqli_file_write_safe
@reports_bp.route("/api/report", methods=["POST"])
def generate_report():
    """Generate a sales report filtered by region."""
    region = request.form.get("region", "all")
    conn = get_db()
    cursor = conn.execute(
        "SELECT id, region, amount FROM sales WHERE region = ?", (region,)
    )  # vuln-code-snippet safe-line chain_sqli_file_write_safe
    rows = cursor.fetchall()
    conn.close()
    return jsonify({"results": [dict(r) for r in rows]})
# vuln-code-snippet end chain_sqli_file_write_safe


init_db()
