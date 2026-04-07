"""Route definitions for login-CSRF scenario -- VULNERABLE variant.

POST /login authenticates a user and creates a session.  Without a CSRF token
on the login form, an attacker can forge a login request from a cross-origin
page, binding the victim's browser session to the attacker's credentials.

Chain: attacker page -> forged POST /login with attacker creds (no CSRF check)
       -> victim browser bound to attacker account
Individual findings: missing CSRF on login (low-medium)
Chain finding: login-CSRF -> session fixation / privacy leakage (medium)
"""
import sqlite3
import hashlib
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
        "(id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT UNIQUE, password TEXT)"
    )
    conn.commit()
    conn.close()


@app.route("/login", methods=["GET", "POST"])
def login():
    """Log in with username and password."""
    if request.method == "GET":
        token = generate_csrf_token()
        return jsonify({"csrf_token": token})

    username = request.form.get("username", "").strip()
    password = request.form.get("password", "")
    token = request.form.get("csrf_token", "")

# vuln-code-snippet start chain_csrf_login_vuln
    if not validate_csrf(token):
        return jsonify({"error": "Invalid CSRF token"}), 403
    if not username or not password:
        return jsonify({"error": "username and password required"}), 400
    hashed = hashlib.sha256(password.encode()).hexdigest()
    conn = get_db()
    row = conn.execute(
        "SELECT id FROM users WHERE username = ? AND password = ?",
        (username, hashed),
    ).fetchone()
    conn.close()
    if not row:
        return jsonify({"error": "Invalid credentials"}), 401
    session.clear()
    session["user_id"] = row["id"]
    session["username"] = username  # vuln-code-snippet vuln-line chain_csrf_login_vuln
# vuln-code-snippet end chain_csrf_login_vuln

    return jsonify({"status": "logged in"})


init_db()

if __name__ == "__main__":
    app.run(port=5000)
