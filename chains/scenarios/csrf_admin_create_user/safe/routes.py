"""Route definitions for admin-user-creation CSRF scenario -- SAFE variant.

POST /admin/users rejects requests without a valid CSRF token, blocking
forged cross-origin user-creation attempts.

Chain: attacker page -> forged POST /admin/users -> 403 (token mismatch) -> blocked
Individual findings: none
Chain finding: none -- CSRF protection is enforced
"""
import sqlite3
from flask import request, jsonify, session
from app import app
from csrf import generate_csrf_token, validate_csrf

DB_PATH = "users.db"


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


def init_db():
    conn = get_db()
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users "
        "(id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT UNIQUE, role TEXT)"
    )
    conn.commit()
    conn.close()


@app.route("/admin/users", methods=["GET", "POST"])
def create_admin_user():
    """Create a new user account (admin only)."""
    if session.get("role") != "admin":
        return jsonify({"error": "Forbidden"}), 403

    if request.method == "GET":
        token = generate_csrf_token()
        return jsonify({"csrf_token": token})

    username = request.form.get("username", "").strip()
    role = request.form.get("role", "user")
    if not username:
        return jsonify({"error": "username required"}), 400

    token = request.form.get("csrf_token", "")

# vuln-code-snippet start chain_csrf_admin_create_safe
    if not validate_csrf(token):
        return jsonify({"error": "Invalid CSRF token"}), 403
    conn = get_db()
    conn.execute(
        "INSERT INTO users (username, role) VALUES (?, ?)",
        (username, role),
    )
    conn.commit()  # vuln-code-snippet safe-line chain_csrf_admin_create_safe
    conn.close()
# vuln-code-snippet end chain_csrf_admin_create_safe

    return jsonify({"status": "user created", "username": username})


init_db()

if __name__ == "__main__":
    app.run(port=5000)
