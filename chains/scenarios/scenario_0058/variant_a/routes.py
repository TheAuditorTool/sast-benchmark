import sqlite3
import hashlib
from flask import request, redirect, jsonify, session
from app import app
from redirect_utils import is_safe_redirect_url

DB_PATH = "users.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def init_db():
    conn = get_db()
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users "
        "(id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT UNIQUE, password TEXT)"
    )
    conn.commit()
    conn.close()

@app.route("/login", methods=["GET", "POST"])
def login():
    if request.method == "GET":
        return jsonify({"message": "POST credentials to /login"})

    username = request.form.get("username", "").strip()
    password = request.form.get("password", "")
    if not username or not password:
        return jsonify({"error": "username and password required"}), 400

    hashed = hashlib.sha256(password.encode()).hexdigest()
    conn = get_db()
    row = conn.execute(
        "SELECT id FROM users WHERE username = ? AND password = ?",
        (username, hashed),
    ).fetchone()
    conn.close()

    if not row:
        return jsonify({"error": "Invalid credentials"}), 401

    session["user_id"] = row["id"]
    session["username"] = username

    next_url = request.args.get("next", "/dashboard")

# vuln-code-snippet start ChainScenario0058A
    if not is_safe_redirect_url(next_url):
        next_url = "/dashboard"
    return redirect(next_url)  # vuln-code-snippet target-line ChainScenario0058A
# vuln-code-snippet end ChainScenario0058A

init_db()

if __name__ == "__main__":
    app.run(port=5000)
