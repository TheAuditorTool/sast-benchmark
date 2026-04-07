import sqlite3
from flask import request, jsonify, session
from module_a import app
from module_b import generate_csrf_token, validate_csrf

DB_PATH = "users.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def init_db():
    conn = get_db()
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users "
        "(id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT UNIQUE)"
    )
    conn.commit()
    conn.close()

@app.route("/account/delete", methods=["GET", "POST"])
def delete_account():
    if not session.get("user_id"):
        return jsonify({"error": "Not authenticated"}), 401

    if request.method == "GET":
        token = generate_csrf_token()
        return jsonify({"csrf_token": token})

    token = request.form.get("csrf_token", "")

    if not validate_csrf(token):
        return jsonify({"error": "Invalid CSRF token"}), 403
    conn = get_db()
    conn.execute("DELETE FROM users WHERE id = ?", (session["user_id"],))
    conn.commit()
    conn.close()

    session.clear()
    return jsonify({"status": "account deleted"})

init_db()

if __name__ == "__main__":
    app.run(port=5000)
