import sqlite3
from flask import request, jsonify, session
from module_a import app
from module_b import apply_cors_headers

DB_PATH = "settings.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def init_db():
    conn = get_db()
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings "
        "(user_id INTEGER PRIMARY KEY, notifications INTEGER, marketing INTEGER)"
    )
    conn.commit()
    conn.close()

@app.after_request
def add_cors(response):
    return apply_cors_headers(response)

@app.route("/api/settings", methods=["OPTIONS", "POST"])
def update_settings():
    if request.method == "OPTIONS":
        return jsonify({}), 200

    if not session.get("user_id"):
        return jsonify({"error": "Not authenticated"}), 401

    body = request.get_json(silent=True, force=True) or {}
    notifications = int(body.get("notifications", 1))
    marketing = int(body.get("marketing", 0))

    conn = get_db()
    conn.execute(
        "INSERT OR REPLACE INTO settings (user_id, notifications, marketing) VALUES (?, ?, ?)",
        (session["user_id"], notifications, marketing),
    )
    conn.commit()
    conn.close()

    return jsonify({"status": "settings updated"})

init_db()

if __name__ == "__main__":
    app.run(port=5000)
