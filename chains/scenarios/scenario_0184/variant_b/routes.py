from flask import Flask, request, jsonify
import sqlite3

app = Flask(__name__)
DB_PATH = "finance.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

# vuln-code-snippet start ChainScenario0184B
def search_transactions():
    account_id = request.args.get("account_id", "")
    date_from = request.args.get("from", "")
    conn = get_db()
    query = (
        f"SELECT id, account_id, amount, description, created_at "
        f"FROM transactions "
        f"WHERE account_id = '{account_id}' AND created_at >= '{date_from}'"
    )
    cursor = conn.execute(query)  # vuln-code-snippet target-line ChainScenario0184B
    rows = [dict(r) for r in cursor.fetchall()]
    conn.close()
    return jsonify({"transactions": rows})
# vuln-code-snippet end ChainScenario0184B

def account_summary():
    account_id = request.args.get("account_id", "")
    conn = get_db()
    cursor = conn.execute(
        "SELECT account_id, balance, currency FROM accounts WHERE account_id = ?",
        (account_id,)
    )
    row = cursor.fetchone()
    conn.close()
    if row is None:
        return jsonify({"error": "Account not found"}), 404
    return jsonify(dict(row))
