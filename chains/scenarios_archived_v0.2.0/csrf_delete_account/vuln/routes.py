"""Route definitions for account-deletion CSRF scenario -- VULNERABLE variant.

POST /account/delete permanently removes the authenticated user's account.
Without CSRF protection, an attacker can embed a hidden form to delete the
victim's account while they browse a malicious page.

Chain: attacker page -> forged POST /account/delete (no CSRF check) -> account deleted
Individual findings: missing CSRF check (medium)
Chain finding: CSRF -> irreversible account destruction (high)
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
        "(id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT UNIQUE)"
    )
    conn.commit()
    conn.close()


@app.route("/account/delete", methods=["GET", "POST"])
def delete_account():
    """Permanently delete the current user's account."""
    if not session.get("user_id"):
        return jsonify({"error": "Not authenticated"}), 401

    if request.method == "GET":
        token = generate_csrf_token()
        return jsonify({"csrf_token": token})

    token = request.form.get("csrf_token", "")

# vuln-code-snippet start chain_csrf_delete_vuln
    if not validate_csrf(token):
        return jsonify({"error": "Invalid CSRF token"}), 403
    conn = get_db()
    conn.execute("DELETE FROM users WHERE id = ?", (session["user_id"],))
    conn.commit()  # vuln-code-snippet vuln-line chain_csrf_delete_vuln
    conn.close()
# vuln-code-snippet end chain_csrf_delete_vuln

    session.clear()
    return jsonify({"status": "account deleted"})


init_db()

if __name__ == "__main__":
    app.run(port=5000)
