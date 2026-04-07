import sqlite3
from flask import Blueprint, jsonify

user_service_bp = Blueprint("user_service", __name__)
DB_PATH = "users.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def init_db():
    conn = get_db()
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users "
        "(id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT, role TEXT, "
        " salary REAL, ssn TEXT)"
    )
    conn.execute(
        "INSERT OR IGNORE INTO users (id, username, role, salary, ssn) "
        "VALUES (1, 'admin', 'admin', 250000.0, '123-45-6789')"
    )
    conn.commit()
    conn.close()

@user_service_bp.route("/api/user/<username>")
def get_user(username):
    conn = get_db()
    row = conn.execute(
        "SELECT id, username, role, salary, ssn FROM users WHERE username = ?",
        (username,),
    ).fetchone()
    conn.close()
    if not row:
        return jsonify({"error": "user not found"}), 404
    return jsonify(dict(row))

init_db()
