from datetime import datetime, timedelta
from module_c import get_db, get_counter, WINDOW_SECONDS, MAX_REQUESTS

def handle_api_request(user_id, payload):
    count, window_start = get_counter(user_id)

    now = datetime.utcnow()
    if window_start:
        ws = datetime.fromisoformat(window_start)
        if now - ws > timedelta(seconds=WINDOW_SECONDS):
            count = 0

    if count >= MAX_REQUESTS:
        return {"error": "Rate limit exceeded"}, 429

    conn = get_db()

    conn.execute(
        """
        INSERT INTO rate_counters (user_id, count, window_start)
        VALUES (?, 1, datetime('now'))
        ON CONFLICT(user_id) DO UPDATE SET count = count + 1
        """,
        (user_id,),
    )

    conn.commit()
    conn.close()

    return {"status": "ok", "echo": payload}, 200
