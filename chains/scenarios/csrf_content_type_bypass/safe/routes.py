"""Route definitions for Content-Type CSRF bypass scenario -- SAFE variant.

POST /settings/profile strictly requires application/json Content-Type and
returns 415 Unsupported Media Type for any other encoding.  An HTML form
submission will be rejected before any data is read.

Chain: attacker form (application/x-www-form-urlencoded) -> POST /settings/profile
       -> 415 Unsupported Media Type -> blocked
Individual findings: none
Chain finding: none -- form encoding is rejected
"""
import sqlite3
from flask import request, jsonify, session
from app import app

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
    """Update the current user's profile."""
    if not session.get("user_id"):
        return jsonify({"error": "Not authenticated"}), 401

# vuln-code-snippet start chain_csrf_content_type_safe
    if not request.is_json:
        return jsonify({"error": "application/json required"}), 415
    data = request.get_json(silent=True) or {}
    display_name = data.get("display_name", "").strip()
    bio = data.get("bio", "").strip()
    conn = get_db()
    conn.execute(
        "INSERT OR REPLACE INTO profiles (user_id, display_name, bio) VALUES (?, ?, ?)",
        (session["user_id"], display_name, bio),
    )
    conn.commit()  # vuln-code-snippet safe-line chain_csrf_content_type_safe
    conn.close()
# vuln-code-snippet end chain_csrf_content_type_safe

    return jsonify({"status": "profile updated"})


init_db()

if __name__ == "__main__":
    app.run(port=5000)
