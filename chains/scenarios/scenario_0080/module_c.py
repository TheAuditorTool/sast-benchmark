import sqlite3
from flask import request, jsonify, session
from module_a import app
from module_b import generate_csrf_token, validate_csrf

DB_PATH = "webhooks.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def init_db():
    conn = get_db()
    conn.execute(
        "CREATE TABLE IF NOT EXISTS webhooks "
        "(id INTEGER PRIMARY KEY AUTOINCREMENT, user_id INTEGER, url TEXT, events TEXT)"
    )
    conn.commit()
    conn.close()

@app.route("/webhooks", methods=["GET", "POST"])
def register_webhook():
    if not session.get("user_id"):
        return jsonify({"error": "Not authenticated"}), 401

    if request.method == "GET":
        token = generate_csrf_token()
        return jsonify({"csrf_token": token})

    url = request.form.get("url", "").strip()
    events = request.form.get("events", "all")
    if not url or not url.startswith("https://"):
        return jsonify({"error": "HTTPS URL required"}), 400

    token = request.form.get("csrf_token", "")

    if not validate_csrf(token):
        return jsonify({"error": "Invalid CSRF token"}), 403
    conn = get_db()
    conn.execute(
        "INSERT INTO webhooks (user_id, url, events) VALUES (?, ?, ?)",
        (session["user_id"], url, events),
    )
    conn.commit()
    conn.close()

    return jsonify({"status": "webhook registered", "url": url})

init_db()

if __name__ == "__main__":
    app.run(port=5000)
