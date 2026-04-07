import sqlite3
import os
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
        "CREATE TABLE IF NOT EXISTS api_keys "
        "(user_id INTEGER PRIMARY KEY, api_key TEXT)"
    )
    conn.commit()
    conn.close()

@app.route("/account/api-key/regenerate", methods=["GET", "POST"])
def regenerate_api_key():
    if not session.get("user_id"):
        return jsonify({"error": "Not authenticated"}), 401

    if request.method == "GET":
        token = generate_csrf_token()
        return jsonify({"csrf_token": token})

    token = request.form.get("csrf_token", "")

    if not validate_csrf(token):
        return jsonify({"error": "Invalid CSRF token"}), 403
    new_key = os.urandom(32).hex()
    conn = get_db()
    conn.execute(
        "INSERT OR REPLACE INTO api_keys (user_id, api_key) VALUES (?, ?)",
        (session["user_id"], new_key),
    )
    conn.commit()
    conn.close()

    return jsonify({"status": "api key regenerated", "api_key": new_key})

init_db()

if __name__ == "__main__":
    app.run(port=5000)
