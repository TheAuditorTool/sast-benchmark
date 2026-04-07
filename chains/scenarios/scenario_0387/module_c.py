import sqlite3
from flask import request, jsonify, session
from module_a import app

DB_PATH = "users.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def init_db():
    conn = get_db()
    conn.execute(
        "CREATE TABLE IF NOT EXISTS profiles "
        "(user_id INTEGER PRIMARY KEY, display_name TEXT, bio TEXT)"
    )
    conn.commit()
    conn.close()

@app.route("/settings/profile", methods=["POST"])
def update_profile():
    if not session.get("user_id"):
        return jsonify({"error": "Not authenticated"}), 401

    data = request.get_json(silent=True)
    if data is None:
        data = request.form  
    display_name = data.get("display_name", "").strip()
    bio = data.get("bio", "").strip()
    conn = get_db()
    conn.execute(
        "INSERT OR REPLACE INTO profiles (user_id, display_name, bio) VALUES (?, ?, ?)",
        (session["user_id"], display_name, bio),
    )
    conn.commit()
    conn.close()

    return jsonify({"status": "profile updated"})

init_db()

if __name__ == "__main__":
    app.run(port=5000)
