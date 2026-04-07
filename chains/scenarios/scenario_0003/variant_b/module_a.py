from flask import Flask, request, jsonify
import sqlite3

app = Flask(__name__)
DB_PATH = "app.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def admin_search_users():
    search = request.args.get("q", "")
    conn = get_db()
    
    query = f"SELECT id, username, email FROM users WHERE username LIKE '%{search}%'"
    cursor = conn.execute(query)
    results = [dict(row) for row in cursor.fetchall()]
    conn.close()
    return jsonify(results)

def admin_dashboard():
    return jsonify({"status": "ok", "page": "admin_dashboard"})
