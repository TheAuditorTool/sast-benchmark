from datetime import datetime, timedelta
from rate_limiter import get_db, WINDOW_SECONDS, MAX_REQUESTS

def handle_api_request(user_id, payload):
    conn = get_db()

# vuln-code-snippet start ChainScenario0219B
    row = conn.execute(
        """
        INSERT INTO rate_counters (user_id, count, window_start)
        VALUES (?, 1, datetime('now'))
        ON CONFLICT(user_id) DO UPDATE SET count = count + 1
        RETURNING count
        """,
        (user_id,),
    ).fetchone()  # vuln-code-snippet target-line ChainScenario0219B
# vuln-code-snippet end ChainScenario0219B

    conn.commit()
    new_count = row["count"] if row else 1

    conn.close()

    if new_count > MAX_REQUESTS:
        return {"error": "Rate limit exceeded"}, 429

    return {"status": "ok", "echo": payload}, 200
