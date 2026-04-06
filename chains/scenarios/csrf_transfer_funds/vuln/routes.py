"""Route definitions for fund-transfer CSRF scenario -- VULNERABLE variant.

POST /transfer moves funds from the authenticated user to a named recipient.
Because CSRF validation is a no-op, an attacker can embed a hidden form on an
external page and drain the victim's balance while they are logged in.

Chain: attacker page -> forged POST /transfer (no CSRF check) -> funds transferred
Individual findings: missing CSRF check (medium)
Chain finding: CSRF -> unauthorized financial transfer (critical)
"""
import sqlite3
from flask import request, jsonify, session
from app import app
from csrf import generate_csrf_token, validate_csrf

DB_PATH = "ledger.db"


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


def init_db():
    conn = get_db()
    conn.execute(
        "CREATE TABLE IF NOT EXISTS accounts "
        "(id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT UNIQUE, balance REAL)"
    )
    conn.execute(
        "CREATE TABLE IF NOT EXISTS transfers "
        "(id INTEGER PRIMARY KEY AUTOINCREMENT, sender TEXT, recipient TEXT, amount REAL)"
    )
    conn.commit()
    conn.close()


@app.route("/transfer", methods=["GET", "POST"])
def transfer_funds():
    """Initiate a fund transfer from the current user's account."""
    if not session.get("user_id"):
        return jsonify({"error": "Not authenticated"}), 401

    if request.method == "GET":
        token = generate_csrf_token()
        return jsonify({"csrf_token": token})

    recipient = request.form.get("recipient", "").strip()
    try:
        amount = float(request.form.get("amount", "0"))
    except ValueError:
        return jsonify({"error": "Invalid amount"}), 400

    if not recipient or amount <= 0:
        return jsonify({"error": "recipient and positive amount required"}), 400

    token = request.form.get("csrf_token", "")

# vuln-code-snippet start chain_csrf_transfer_vuln
    if not validate_csrf(token):
        return jsonify({"error": "Invalid CSRF token"}), 403
    conn = get_db()
    conn.execute(
        "UPDATE accounts SET balance = balance - ? WHERE username = ?",
        (amount, session["username"]),
    )
    conn.execute(
        "UPDATE accounts SET balance = balance + ? WHERE username = ?",
        (amount, recipient),
    )
    conn.execute(
        "INSERT INTO transfers (sender, recipient, amount) VALUES (?, ?, ?)",
        (session["username"], recipient, amount),
    )
    conn.commit()  # vuln-code-snippet vuln-line chain_csrf_transfer_vuln
    conn.close()
# vuln-code-snippet end chain_csrf_transfer_vuln

    return jsonify({"status": "transfer complete", "amount": amount, "to": recipient})


init_db()

if __name__ == "__main__":
    app.run(port=5000)
