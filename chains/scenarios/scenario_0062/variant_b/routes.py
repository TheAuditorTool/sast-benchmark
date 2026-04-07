from flask import Flask, request, jsonify
import sqlite3

app = Flask(__name__)
DB_PATH = "users.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

# vuln-code-snippet start ChainScenario0062B
def search_users():
    term = request.args.get("username", "")
    conn = get_db()
    query = f"SELECT id, username, email, role FROM users WHERE username LIKE '%{term}%'"
    cursor = conn.execute(query)  # vuln-code-snippet target-line ChainScenario0062B
    rows = [dict(r) for r in cursor.fetchall()]
    conn.close()
    return jsonify({"users": rows})
# vuln-code-snippet end ChainScenario0062B

def list_users():
    conn = get_db()
    cursor = conn.execute("SELECT id, username, email, role FROM users")
    rows = [dict(r) for r in cursor.fetchall()]
    conn.close()
    return jsonify({"users": rows})
