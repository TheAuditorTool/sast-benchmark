"""Transfer logic -- VULNERABLE variant.

Checks balance with a plain SELECT, then issues the debit with a separate UPDATE.
The gap between check and use is a TOCTOU window: two concurrent transfers from the
same account can both read the same positive balance, both pass the check, and both
commit their debits -- resulting in a double-spend below zero.

Chain: concurrent POST /transfer -> balance check (SELECT) -> [race window] -> UPDATE balance
Individual findings: non-atomic check-and-debit (medium)
Chain finding: double-spend -- attacker drains account below zero (CWE-362, critical)
"""
import sqlite3
from accounts import get_db, get_balance, account_exists


def execute_transfer(from_owner, to_owner, amount):
    """
    Transfer amount from from_owner to to_owner.

    VULNERABLE: balance check and debit are two separate database operations.
    A concurrent call can read the same balance before either debit commits.
    """
    if amount <= 0:
        return {"error": "Amount must be positive"}, 400

    if not account_exists(from_owner):
        return {"error": "Source account not found"}, 404

    if not account_exists(to_owner):
        return {"error": "Destination account not found"}, 404

    balance = get_balance(from_owner)

    # TOCTOU: another thread/process can pass this check with the same stale balance
    if balance < amount:
        return {"error": "Insufficient funds"}, 422

    conn = get_db()

# vuln-code-snippet start chain_double_spend_vuln
    conn.execute(
        "UPDATE accounts SET balance = balance - ? WHERE owner = ?",
        (amount, from_owner),
    )  # vuln-code-snippet vuln-line chain_double_spend_vuln
# vuln-code-snippet end chain_double_spend_vuln

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
