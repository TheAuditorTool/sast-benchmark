"""Route definitions for email-change CSRF scenario -- VULNERABLE variant.

POST /account/email accepts any request. An attacker can forge a cross-origin
POST to change the victim's email to an attacker-controlled address, enabling
account takeover via password-reset flow.

Chain: attacker page -> forged POST /account/email (no CSRF check) -> email changed
Individual findings: missing CSRF check (medium)
Chain finding: CSRF -> account takeover via email hijack (critical)
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
        "(id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT UNIQUE, email TEXT)"
    )
    conn.commit()
    conn.close()


@app.route("/account/email", methods=["GET", "POST"])
def change_email():
    """Update the current user's email address."""
    if not session.get("user_id"):
        return jsonify({"error": "Not authenticated"}), 401

    if request.method == "GET":
        token = generate_csrf_token()
        return jsonify({"csrf_token": token})

    new_email = request.form.get("email", "").strip()
    if not new_email or "@" not in new_email:
        return jsonify({"error": "Valid email required"}), 400

    token = request.form.get("csrf_token", "")

# vuln-code-snippet start chain_csrf_email_vuln
    if not validate_csrf(token):
        return jsonify({"error": "Invalid CSRF token"}), 403
    conn = get_db()
    conn.execute(
        "UPDATE users SET email = ? WHERE id = ?",
        (new_email, session["user_id"]),
    )
    conn.commit()  # vuln-code-snippet vuln-line chain_csrf_email_vuln
    conn.close()
# vuln-code-snippet end chain_csrf_email_vuln

    return jsonify({"status": "email updated"})


init_db()

if __name__ == "__main__":
    app.run(port=5000)
