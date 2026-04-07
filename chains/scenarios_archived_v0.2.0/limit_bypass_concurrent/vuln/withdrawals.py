"""Withdrawal processing logic -- VULNERABLE variant.

Reads the current daily total with SELECT, checks it against the limit,
then increments with a separate UPSERT.  Two concurrent withdrawals of 600
each both read total=0 (under the 1000 limit), both pass, and both commit --
resulting in a daily total of 1200.

Chain: concurrent POST /withdraw -> SELECT total -> [race window] -> UPSERT total + amount
Individual findings: non-atomic limit check-and-increment (medium)
Chain finding: daily limit bypassed via concurrent withdrawals (CWE-362, critical)
"""
from datetime import date
from limits import get_db, get_daily_total, DAILY_LIMIT


def process_withdrawal(user_id, amount):
    """
    Process a withdrawal for user_id.

    VULNERABLE: daily total is read and updated in two separate statements.
    Concurrent withdrawals can both pass the limit check before either updates.
    """
    if amount <= 0:
        return {"error": "Amount must be positive"}, 400

    today = date.today().isoformat()
    current_total = get_daily_total(user_id, today)

    # TOCTOU: concurrent withdrawals can both read the same stale total
    if current_total + amount > DAILY_LIMIT:
        return {"error": "Daily withdrawal limit exceeded"}, 422

    conn = get_db()

# vuln-code-snippet start chain_limit_bypass_vuln
    conn.execute(
        """
        INSERT INTO daily_totals (user_id, date, total) VALUES (?, ?, ?)
        ON CONFLICT(user_id, date) DO UPDATE SET total = total + ?
        """,
        (user_id, today, amount, amount),
    )  # vuln-code-snippet vuln-line chain_limit_bypass_vuln
# vuln-code-snippet end chain_limit_bypass_vuln

    conn.execute(
        "INSERT INTO withdrawal_log (user_id, amount) VALUES (?, ?)",
        (user_id, amount),
    )
    conn.commit()
    conn.close()
    return {"status": "ok", "amount": amount}, 200
