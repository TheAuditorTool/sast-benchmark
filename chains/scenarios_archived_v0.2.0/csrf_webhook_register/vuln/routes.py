"""Route definitions for webhook-registration CSRF scenario -- VULNERABLE variant.

POST /webhooks registers a callback URL that will receive event payloads for the
current user's account.  Without CSRF protection an attacker can register their
own server as the callback, silently receiving all future event data.

Chain: attacker page -> forged POST /webhooks (no CSRF check) -> attacker endpoint registered
Individual findings: missing CSRF check (medium)
Chain finding: CSRF -> sensitive data exfiltration via webhook hijack (high)
"""
import sqlite3
from flask import request, jsonify, session
from app import app
from csrf import generate_csrf_token, validate_csrf

DB_PATH = "webhooks.db"


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


def init_db():
    conn = get_db()
    conn.execute(
        "CREATE TABLE IF NOT EXISTS webhooks "
        "(id INTEGER PRIMARY KEY AUTOINCREMENT, user_id INTEGER, url TEXT, events TEXT)"
    )
    conn.commit()
    conn.close()


@app.route("/webhooks", methods=["GET", "POST"])
def register_webhook():
    """Register a new webhook for the current user."""
    if not session.get("user_id"):
        return jsonify({"error": "Not authenticated"}), 401

    if request.method == "GET":
        token = generate_csrf_token()
        return jsonify({"csrf_token": token})

    url = request.form.get("url", "").strip()
    events = request.form.get("events", "all")
    if not url or not url.startswith("https://"):
        return jsonify({"error": "HTTPS URL required"}), 400

    token = request.form.get("csrf_token", "")

# vuln-code-snippet start chain_csrf_webhook_vuln
    if not validate_csrf(token):
        return jsonify({"error": "Invalid CSRF token"}), 403
    conn = get_db()
    conn.execute(
        "INSERT INTO webhooks (user_id, url, events) VALUES (?, ?, ?)",
        (session["user_id"], url, events),
    )
    conn.commit()  # vuln-code-snippet vuln-line chain_csrf_webhook_vuln
    conn.close()
# vuln-code-snippet end chain_csrf_webhook_vuln

    return jsonify({"status": "webhook registered", "url": url})


init_db()

if __name__ == "__main__":
    app.run(port=5000)
