"""Route definitions for login-redirect open redirect scenario -- VULNERABLE variant.

POST /login reads a ?next= query parameter and redirects there after successful
authentication.  Because is_safe_redirect_url() always returns True, an attacker
can craft a phishing link: /login?next=https://evil.com/steal-creds

Chain: user follows /login?next=https://evil.com -> authenticates -> redirected to phishing page
Individual findings: unvalidated redirect parameter (medium)
Chain finding: open redirect on login -> credential phishing (high)
"""
import sqlite3
import hashlib
from flask import request, redirect, jsonify, session
from app import app
from redirect_utils import is_safe_redirect_url

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
    """Authenticate and redirect to ?next= if supplied."""
    if request.method == "GET":
        return jsonify({"message": "POST credentials to /login"})

    username = request.form.get("username", "").strip()
    password = request.form.get("password", "")
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

    session["user_id"] = row["id"]
    session["username"] = username

    next_url = request.args.get("next", "/dashboard")

# vuln-code-snippet start chain_login_redirect_vuln
    if not is_safe_redirect_url(next_url):
        next_url = "/dashboard"
    return redirect(next_url)  # vuln-code-snippet vuln-line chain_login_redirect_vuln
# vuln-code-snippet end chain_login_redirect_vuln


init_db()

if __name__ == "__main__":
    app.run(port=5000)
