"""Protected API endpoint logic -- SAFE variant.

Uses a single atomic UPSERT that increments the counter and returns the new value.
The RETURNING clause delivers the post-increment count so the check happens after
the increment -- if the new count exceeds the limit the request is rejected even
when concurrent callers race.

Chain: concurrent requests -> atomic UPSERT (count + 1) -> post-increment check
Individual findings: none -- increment and check are atomic
Chain finding: none -- concurrent burst is correctly capped (CWE-362 mitigated)
"""
from datetime import datetime, timedelta
from rate_limiter import get_db, WINDOW_SECONDS, MAX_REQUESTS


def handle_api_request(user_id, payload):
    """
    Process a rate-limited API request for user_id.

    SAFE: the counter is incremented atomically via UPSERT; the returned new count
    is checked after the fact.  Concurrent callers each get a distinct post-increment
    value, so only the first MAX_REQUESTS are permitted per window.
    """
    conn = get_db()

# vuln-code-snippet start chain_rate_limit_race_safe
    row = conn.execute(
        """
        INSERT INTO rate_counters (user_id, count, window_start)
        VALUES (?, 1, datetime('now'))
        ON CONFLICT(user_id) DO UPDATE SET count = count + 1
        RETURNING count
        """,
        (user_id,),
    ).fetchone()  # vuln-code-snippet safe-line chain_rate_limit_race_safe
# vuln-code-snippet end chain_rate_limit_race_safe

    conn.commit()
    new_count = row["count"] if row else 1

    conn.close()

    if new_count > MAX_REQUESTS:
        return {"error": "Rate limit exceeded"}, 429

    return {"status": "ok", "echo": payload}, 200
