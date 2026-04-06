"""Route definitions for SameSite-bypass CSRF scenario -- SAFE variant.

GET /notifications/dismiss is removed; the operation is now POST-only.
SameSite=Lax blocks cookies on cross-site POST sub-resource requests, so
the forged navigation attack no longer works.

Chain: attacker page -> top-level GET /notifications/dismiss -> 405 Method Not Allowed -> blocked
Individual findings: none
Chain finding: none -- state change is POST-only
"""
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


# vuln-code-snippet start chain_csrf_samesite_safe
@app.route("/notifications/dismiss", methods=["POST"])
def dismiss_notification():
    """Dismiss a notification by ID. SAFE: POST-only prevents SameSite=Lax bypass."""
    if not session.get("user_id"):
        return jsonify({"error": "Not authenticated"}), 401
    notification_id = request.form.get("id")
    if not notification_id:
        return jsonify({"error": "id required"}), 400
    conn = get_db()
    conn.execute(
        "UPDATE notifications SET dismissed = 1 WHERE id = ? AND user_id = ?",
        (notification_id, session["user_id"]),
    )
    conn.commit()  # vuln-code-snippet safe-line chain_csrf_samesite_safe
    conn.close()
    return jsonify({"status": "dismissed"})
# vuln-code-snippet end chain_csrf_samesite_safe


init_db()

if __name__ == "__main__":
    app.run(port=5000)
