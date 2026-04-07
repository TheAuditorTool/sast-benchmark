from datetime import date
from module_b import get_db, DAILY_LIMIT

def process_withdrawal(user_id, amount):
    if amount <= 0:
        return {"error": "Amount must be positive"}, 400

    today = date.today().isoformat()
    conn = get_db()

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
    conn.execute("COMMIT")

    conn.close()
    return {"status": "ok", "amount": amount}, 200
