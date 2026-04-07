"""Financial transaction routes with SQL injection.

This file is IDENTICAL between vuln/ and safe/ variants.
The SQL injection exists in both -- what differs is whether
auth.py enforces MFA for API callers (see auth.py).

Chain: API login (no MFA) -> valid session -> SQLi on /finance/transactions
"""
from flask import Flask, request, jsonify
import sqlite3

app = Flask(__name__)
DB_PATH = "finance.db"


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


# vuln-code-snippet start chain_mfa_bypass_sqli_safe
def search_transactions():
    """Search financial transactions. Requires authenticated session."""
    account_id = request.args.get("account_id", "")
    date_from = request.args.get("from", "")
    conn = get_db()
    query = (
        f"SELECT id, account_id, amount, description, created_at "
        f"FROM transactions "
        f"WHERE account_id = '{account_id}' AND created_at >= '{date_from}'"
    )
    cursor = conn.execute(query)  # vuln-code-snippet safe-line chain_mfa_bypass_sqli_safe
    rows = [dict(r) for r in cursor.fetchall()]
    conn.close()
    return jsonify({"transactions": rows})
# vuln-code-snippet end chain_mfa_bypass_sqli_safe


def account_summary():
    """Return account summary. Requires authenticated session."""
    account_id = request.args.get("account_id", "")
    conn = get_db()
    cursor = conn.execute(
        "SELECT account_id, balance, currency FROM accounts WHERE account_id = ?",
        (account_id,)
    )
    row = cursor.fetchone()
    conn.close()
    if row is None:
        return jsonify({"error": "Account not found"}), 404
    return jsonify(dict(row))
