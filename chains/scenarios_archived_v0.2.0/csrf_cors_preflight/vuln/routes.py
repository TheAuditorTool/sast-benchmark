"""Route definitions for CORS-preflight CSRF scenario -- VULNERABLE variant.

POST /api/settings changes notification preferences.  The CORS policy reflects
any Origin with credentials allowed, so a cross-origin page can make credentialed
requests.  No CSRF token is required, so the attacker can read the response AND
trigger state changes from any origin.

Chain: cross-origin page -> credentialed POST /api/settings (CORS allows all origins)
       -> no CSRF check -> settings changed
Individual findings: wildcard CORS with credentials (medium)
Chain finding: CORS misconfiguration + missing CSRF -> cross-origin state change (high)
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

    notifications = int(request.get_json(silent=True, force=True).get("notifications", 1))
    marketing = int(request.get_json(silent=True, force=True).get("marketing", 0))

# vuln-code-snippet start chain_csrf_cors_vuln
    conn = get_db()
    conn.execute(
        "INSERT OR REPLACE INTO settings (user_id, notifications, marketing) VALUES (?, ?, ?)",
        (session["user_id"], notifications, marketing),
    )
    conn.commit()  # vuln-code-snippet vuln-line chain_csrf_cors_vuln
    conn.close()
# vuln-code-snippet end chain_csrf_cors_vuln

    return jsonify({"status": "settings updated"})


init_db()

if __name__ == "__main__":
    app.run(port=5000)
