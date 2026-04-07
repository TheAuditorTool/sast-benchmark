"""Route definitions for CORS-preflight CSRF scenario -- SAFE variant.

CORS headers are only reflected for whitelisted origins.  Cross-origin
credentialed requests from unknown origins are blocked by the browser.

Chain: cross-origin page -> preflight rejected (no ACAO for unknown origin) -> blocked
Individual findings: none
Chain finding: none -- CORS origin validation is enforced
"""
import sqlite3
from flask import request, jsonify, session
from app import app
from csrf import apply_cors_headers

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
    """Apply CORS headers to all responses."""
    return apply_cors_headers(response)


@app.route("/api/settings", methods=["OPTIONS", "POST"])
def update_settings():
    """Update notification settings for the current user."""
    if request.method == "OPTIONS":
        return jsonify({}), 200

    if not session.get("user_id"):
        return jsonify({"error": "Not authenticated"}), 401

    body = request.get_json(silent=True, force=True) or {}
    notifications = int(body.get("notifications", 1))
    marketing = int(body.get("marketing", 0))

# vuln-code-snippet start chain_csrf_cors_safe
    conn = get_db()
    conn.execute(
        "INSERT OR REPLACE INTO settings (user_id, notifications, marketing) VALUES (?, ?, ?)",
        (session["user_id"], notifications, marketing),
    )
    conn.commit()  # vuln-code-snippet safe-line chain_csrf_cors_safe
    conn.close()
# vuln-code-snippet end chain_csrf_cors_safe

    return jsonify({"status": "settings updated"})


init_db()

if __name__ == "__main__":
    app.run(port=5000)
