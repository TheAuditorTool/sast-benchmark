import sqlite3
from flask import request, jsonify, session
from app import app
from csrf import generate_csrf_token, validate_csrf

DB_PATH = "users.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def init_db():
    conn = get_db()
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users "
        "(id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT UNIQUE, role TEXT)"
    )
    conn.commit()
    conn.close()

@app.route("/admin/users", methods=["GET", "POST"])
def create_admin_user():
    if session.get("role") != "admin":
        return jsonify({"error": "Forbidden"}), 403

    if request.method == "GET":
        token = generate_csrf_token()
        return jsonify({"csrf_token": token})

    username = request.form.get("username", "").strip()
    role = request.form.get("role", "user")
    if not username:
        return jsonify({"error": "username required"}), 400

    token = request.form.get("csrf_token", "")

# vuln-code-snippet start ChainScenario0025B
    if not validate_csrf(token):
        return jsonify({"error": "Invalid CSRF token"}), 403
    conn = get_db()
    conn.execute(
        "INSERT INTO users (username, role) VALUES (?, ?)",
        (username, role),
    )
    conn.commit()  # vuln-code-snippet target-line ChainScenario0025B
    conn.close()
# vuln-code-snippet end ChainScenario0025B

    return jsonify({"status": "user created", "username": username})

init_db()

if __name__ == "__main__":
    app.run(port=5000)
