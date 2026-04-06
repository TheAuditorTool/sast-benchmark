"""Route definitions for Content-Type CSRF bypass scenario -- VULNERABLE variant.

POST /settings/profile is intended to accept JSON only, relying on the browser's
CORS preflight behaviour to block cross-site JSON requests.  However the handler
also falls back to reading request.form when JSON parsing returns None, making it
reachable via a plain HTML form submission without any CSRF token.

Chain: attacker form (application/x-www-form-urlencoded) -> POST /settings/profile
       -> form fallback accepted -> profile updated
Individual findings: Content-Type bypass (medium)
Chain finding: CSRF via form encoding -> unauthorized profile change (medium)
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

# vuln-code-snippet start chain_csrf_content_type_vuln
    data = request.get_json(silent=True)
    if data is None:
        data = request.form  # VULN: form fallback bypasses JSON-only assumption
    display_name = data.get("display_name", "").strip()
    bio = data.get("bio", "").strip()
    conn = get_db()
    conn.execute(
        "INSERT OR REPLACE INTO profiles (user_id, display_name, bio) VALUES (?, ?, ?)",
        (session["user_id"], display_name, bio),
    )
    conn.commit()  # vuln-code-snippet vuln-line chain_csrf_content_type_vuln
    conn.close()
# vuln-code-snippet end chain_csrf_content_type_vuln

    return jsonify({"status": "profile updated"})


init_db()

if __name__ == "__main__":
    app.run(port=5000)
