"""Route definitions for SameSite-bypass CSRF scenario -- VULNERABLE variant.

GET /notifications/dismiss?id=<X> marks a notification as dismissed.  The developer
assumed SameSite=Lax would protect this endpoint, but Lax only blocks cookies on
cross-site sub-resource requests -- top-level navigations (window.location, <a href>)
still carry Lax cookies, so an attacker can silently dismiss arbitrary notifications.

Chain: attacker page -> top-level GET /notifications/dismiss?id=X (Lax cookie sent) -> dismissed
Individual findings: state-changing GET endpoint (low)
Chain finding: SameSite=Lax + GET state change -> CSRF (medium)
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


# vuln-code-snippet start chain_csrf_samesite_vuln
@app.route("/notifications/dismiss", methods=["GET"])
def dismiss_notification():
    """Dismiss a notification by ID. VULNERABLE: state change on GET."""
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
    conn.commit()  # vuln-code-snippet vuln-line chain_csrf_samesite_vuln
    conn.close()
    return jsonify({"status": "dismissed"})
# vuln-code-snippet end chain_csrf_samesite_vuln


init_db()

if __name__ == "__main__":
    app.run(port=5000)
