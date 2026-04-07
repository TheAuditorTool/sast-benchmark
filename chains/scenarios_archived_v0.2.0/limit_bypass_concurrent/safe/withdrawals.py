"""Withdrawal processing logic -- SAFE variant.

Uses BEGIN IMMEDIATE to serialize the read-check-update sequence.
The write lock is acquired before reading the daily total, so concurrent
transactions observe each other's committed updates before deciding to proceed.

Chain: concurrent POST /withdraw -> BEGIN IMMEDIATE -> SELECT total -> conditional UPDATE (serialized)
Individual findings: none -- check and update are within a serialized transaction
Chain finding: none -- concurrent withdrawals correctly accumulate against the limit (CWE-362 mitigated)
"""
from datetime import date
from limits import get_db, DAILY_LIMIT


def process_withdrawal(user_id, amount):
    """
    Process a withdrawal for user_id.

    SAFE: BEGIN IMMEDIATE serializes the select-check-insert sequence.
    Each concurrent withdrawal sees the committed total from prior withdrawals.
    """
    if amount <= 0:
        return {"error": "Amount must be positive"}, 400

    today = date.today().isoformat()
    conn = get_db()

# vuln-code-snippet start chain_limit_bypass_safe
    conn.execute("BEGIN IMMEDIATE")
    row = conn.execute(
        "SELECT total FROM daily_totals WHERE user_id = ? AND date = ?",
        (user_id, today),
    ).fetchone()
    current_total = row["total"] if row else 0.0

    if current_total + amount > DAILY_LIMIT:
        conn.execute("ROLLBACK")
        conn.close()
        return {"error": "Daily withdrawal limit exceeded"}, 422

    conn.execute(
        """
        INSERT INTO daily_totals (user_id, date, total) VALUES (?, ?, ?)
        ON CONFLICT(user_id, date) DO UPDATE SET total = total + ?
        """,
        (user_id, today, amount, amount),
    )
    conn.execute(
        "INSERT INTO withdrawal_log (user_id, amount) VALUES (?, ?)",
        (user_id, amount),
    )
    conn.execute("COMMIT")  # vuln-code-snippet safe-line chain_limit_bypass_safe
# vuln-code-snippet end chain_limit_bypass_safe

    conn.close()
    return {"status": "ok", "amount": amount}, 200
