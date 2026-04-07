from datetime import date
from module_b import get_db, get_daily_total, DAILY_LIMIT

def process_withdrawal(user_id, amount):
    if amount <= 0:
        return {"error": "Amount must be positive"}, 400

    today = date.today().isoformat()
    current_total = get_daily_total(user_id, today)

    if current_total + amount > DAILY_LIMIT:
        return {"error": "Daily withdrawal limit exceeded"}, 422

    conn = get_db()

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
    conn.commit()
    conn.close()
    return {"status": "ok", "amount": amount}, 200
