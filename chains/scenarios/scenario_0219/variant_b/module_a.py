from datetime import datetime, timedelta
from module_c import get_db, WINDOW_SECONDS, MAX_REQUESTS

def handle_api_request(user_id, payload):
    conn = get_db()

    row = conn.execute(
        """
        INSERT INTO rate_counters (user_id, count, window_start)
        VALUES (?, 1, datetime('now'))
        ON CONFLICT(user_id) DO UPDATE SET count = count + 1
        RETURNING count
        """,
        (user_id,),
    ).fetchone()

    conn.commit()
    new_count = row["count"] if row else 1

    conn.close()

    if new_count > MAX_REQUESTS:
        return {"error": "Rate limit exceeded"}, 429

    return {"status": "ok", "echo": payload}, 200
