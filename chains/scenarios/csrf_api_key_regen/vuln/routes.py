"""Route definitions for API-key-rotation CSRF scenario -- VULNERABLE variant.

POST /account/api-key/regenerate replaces the user's API key with a new random value.
Without CSRF protection an attacker can invalidate the victim's current key and
potentially observe the new key if it is reflected in the response.

Chain: attacker page -> forged POST /account/api-key/regenerate (no CSRF check) -> key rotated
Individual findings: missing CSRF check (medium)
Chain finding: CSRF -> API key takeover / denial of service (high)
"""
import sqlite3
import os
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
        "CREATE TABLE IF NOT EXISTS api_keys "
        "(user_id INTEGER PRIMARY KEY, api_key TEXT)"
    )
    conn.commit()
    conn.close()


@app.route("/account/api-key/regenerate", methods=["GET", "POST"])
def regenerate_api_key():
    """Regenerate the current user's API key."""
    if not session.get("user_id"):
        return jsonify({"error": "Not authenticated"}), 401

    if request.method == "GET":
        token = generate_csrf_token()
        return jsonify({"csrf_token": token})

    token = request.form.get("csrf_token", "")

# vuln-code-snippet start chain_csrf_api_key_vuln
    if not validate_csrf(token):
        return jsonify({"error": "Invalid CSRF token"}), 403
    new_key = os.urandom(32).hex()
    conn = get_db()
    conn.execute(
        "INSERT OR REPLACE INTO api_keys (user_id, api_key) VALUES (?, ?)",
        (session["user_id"], new_key),
    )
    conn.commit()  # vuln-code-snippet vuln-line chain_csrf_api_key_vuln
    conn.close()
# vuln-code-snippet end chain_csrf_api_key_vuln

    return jsonify({"status": "api key regenerated", "api_key": new_key})


init_db()

if __name__ == "__main__":
    app.run(port=5000)
