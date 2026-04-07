from flask import Flask, request, jsonify
import sqlite3

app = Flask(__name__)
DB_PATH = "app.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

# vuln-code-snippet start ChainScenario0003A
def admin_search_users():
    search = request.args.get("q", "")
    conn = get_db()
    
    query = f"SELECT id, username, email FROM users WHERE username LIKE '%{search}%'"
    cursor = conn.execute(query)  # vuln-code-snippet target-line ChainScenario0003A
    results = [dict(row) for row in cursor.fetchall()]
    conn.close()
    return jsonify(results)
# vuln-code-snippet end ChainScenario0003A

def admin_dashboard():
    return jsonify({"status": "ok", "page": "admin_dashboard"})
