"""Transfer logic -- SAFE variant.

Uses a single atomic UPDATE that checks the balance condition in the WHERE clause.
The database engine evaluates the condition and applies the debit in one operation,
so concurrent transactions cannot both observe a stale pre-debit balance.

Chain: concurrent POST /transfer -> atomic UPDATE WHERE balance >= amount (serialized by DB)
Individual findings: none -- check-and-debit are atomic
Chain finding: none -- double-spend window is eliminated (CWE-362 mitigated)
"""
import sqlite3
from accounts import get_db, get_balance, account_exists


def execute_transfer(from_owner, to_owner, amount):
    """
    Transfer amount from from_owner to to_owner.

    SAFE: the debit UPDATE is conditional on sufficient balance in its own WHERE
    clause, so the check and the mutation are a single atomic database operation.
    """
    if amount <= 0:
        return {"error": "Amount must be positive"}, 400

    if not account_exists(from_owner):
        return {"error": "Source account not found"}, 404

    if not account_exists(to_owner):
        return {"error": "Destination account not found"}, 404

    conn = get_db()

# vuln-code-snippet start chain_double_spend_safe
    cursor = conn.execute(
        "UPDATE accounts SET balance = balance - ? WHERE owner = ? AND balance >= ?",
        (amount, from_owner, amount),
    )  # vuln-code-snippet safe-line chain_double_spend_safe
# vuln-code-snippet end chain_double_spend_safe

    if cursor.rowcount == 0:
        conn.close()
        return {"error": "Insufficient funds"}, 422

    conn.execute(
        "UPDATE accounts SET balance = balance + ? WHERE owner = ?",
        (amount, to_owner),
    )
    conn.execute(
        "INSERT INTO transfers (from_owner, to_owner, amount) VALUES (?, ?, ?)",
        (from_owner, to_owner, amount),
    )
    conn.commit()
    conn.close()

    new_balance = get_balance(from_owner)
    return {"status": "ok", "new_balance": new_balance}, 200
