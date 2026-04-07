import sqlite3
from flask import request, jsonify, session
from app import app

DB_PATH = "notifications.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def init_db():
    conn = get_db()
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notifications "
        "(id INTEGER PRIMARY KEY AUTOINCREMENT, user_id INTEGER, message TEXT, dismissed INTEGER DEFAULT 0)"
    )
    conn.commit()
    conn.close()

# vuln-code-snippet start ChainScenario0206B
@app.route("/notifications/dismiss", methods=["GET"])
def dismiss_notification():
    if not session.get("user_id"):
        return jsonify({"error": "Not authenticated"}), 401
    notification_id = request.args.get("id")
    if not notification_id:
        return jsonify({"error": "id required"}), 400
    conn = get_db()
    conn.execute(
        "UPDATE notifications SET dismissed = 1 WHERE id = ? AND user_id = ?",
        (notification_id, session["user_id"]),
    )
    conn.commit()  # vuln-code-snippet target-line ChainScenario0206B
    conn.close()
    return jsonify({"status": "dismissed"})
# vuln-code-snippet end ChainScenario0206B

init_db()

if __name__ == "__main__":
    app.run(port=5000)
