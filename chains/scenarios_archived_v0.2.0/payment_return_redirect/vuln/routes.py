"""Route definitions for payment-return open redirect scenario -- VULNERABLE variant.

POST /checkout stores a return_url alongside the payment session.
GET /payment/return?session=<S> completes the session and redirects to return_url.
An attacker supplies return_url=https://evil.com; after payment the user lands on
a fake success page that harvests further data.

Chain: POST /checkout {return_url: https://evil.com} -> GET /payment/return?session=S
       -> redirect to phishing page
Individual findings: unvalidated payment return URL (medium)
Chain finding: open redirect after payment -> phishing / credential theft (high)
"""
import sqlite3
import os
from flask import request, redirect, jsonify, session
from app import app
from redirect_utils import is_safe_return_url

DB_PATH = "payments.db"


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


def init_db():
    conn = get_db()
    conn.execute(
        "CREATE TABLE IF NOT EXISTS payment_sessions "
        "(session_id TEXT PRIMARY KEY, user_id INTEGER, amount REAL, return_url TEXT, status TEXT)"
    )
    conn.commit()
    conn.close()


@app.route("/checkout", methods=["POST"])
def checkout():
    """Initiate a payment session."""
    if not session.get("user_id"):
        return jsonify({"error": "Not authenticated"}), 401

    body = request.get_json(silent=True) or request.form
    amount = body.get("amount")
    return_url = body.get("return_url", "/orders")

    if not amount:
        return jsonify({"error": "amount required"}), 400

    if not is_safe_return_url(return_url):
        return jsonify({"error": "Invalid return URL"}), 400

    session_id = os.urandom(16).hex()
    conn = get_db()
    conn.execute(
        "INSERT INTO payment_sessions (session_id, user_id, amount, return_url, status) "
        "VALUES (?, ?, ?, ?, ?)",
        (session_id, session["user_id"], amount, return_url, "pending"),
    )
    conn.commit()
    conn.close()
    return jsonify({"session_id": session_id})


@app.route("/payment/return")
def payment_return():
    """Complete a payment session and redirect to the merchant return URL."""
    session_id = request.args.get("session", "")
    if not session_id:
        return jsonify({"error": "session required"}), 400

    conn = get_db()
    row = conn.execute(
        "SELECT return_url FROM payment_sessions WHERE session_id = ?", (session_id,)
    ).fetchone()
    conn.close()

    if not row:
        return jsonify({"error": "Invalid session"}), 400

# vuln-code-snippet start chain_payment_redirect_vuln
    return redirect(row["return_url"])  # vuln-code-snippet vuln-line chain_payment_redirect_vuln
# vuln-code-snippet end chain_payment_redirect_vuln


init_db()

if __name__ == "__main__":
    app.run(port=5000)
