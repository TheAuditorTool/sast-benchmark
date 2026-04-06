"""Route definitions for email-verification open redirect scenario -- VULNERABLE variant.

GET /verify?token=<T>&next=<URL> validates the token, marks the email as verified,
then redirects to ?next=.  An attacker who can embed a crafted link in the
verification email (e.g. via email injection) can route the user to a phishing page.

Chain: GET /verify?token=valid&next=https://evil.com -> verified -> redirect to phishing page
Individual findings: unvalidated redirect after email verification (medium)
Chain finding: open redirect on verification link -> phishing (medium)
"""
import sqlite3
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
        "(id INTEGER PRIMARY KEY, username TEXT, email_verified INTEGER DEFAULT 0, verify_token TEXT)"
    )
    conn.commit()
    conn.close()


@app.route("/verify")
def verify_email():
    """Validate email token and redirect to next URL."""
    token = request.args.get("token", "")
    next_url = request.args.get("next", "/dashboard")

    if not token:
        return jsonify({"error": "token required"}), 400

    conn = get_db()
    row = conn.execute(
        "SELECT id FROM users WHERE verify_token = ?", (token,)
    ).fetchone()

    if not row:
        conn.close()
        return jsonify({"error": "Invalid or expired token"}), 400

    conn.execute(
        "UPDATE users SET email_verified = 1, verify_token = NULL WHERE id = ?",
        (row["id"],),
    )
    conn.commit()
    conn.close()

    session["user_id"] = row["id"]

# vuln-code-snippet start chain_email_verify_redirect_vuln
    if not is_safe_redirect_url(next_url):
        next_url = "/dashboard"
    return redirect(next_url)  # vuln-code-snippet vuln-line chain_email_verify_redirect_vuln
# vuln-code-snippet end chain_email_verify_redirect_vuln


init_db()

if __name__ == "__main__":
    app.run(port=5000)
