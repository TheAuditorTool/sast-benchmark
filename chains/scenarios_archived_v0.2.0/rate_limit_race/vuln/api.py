"""Protected API endpoint logic -- VULNERABLE variant.

Reads the current request count with SELECT, decides whether to allow the request,
then increments with a separate UPDATE.  N concurrent requests all read count=9
(one below the limit), all pass the check, and all increment -- resulting in
count=19 instead of the enforced limit of 10.

Chain: concurrent requests -> SELECT count -> [race window] -> UPDATE count + 1
Individual findings: non-atomic rate check-and-increment (medium)
Chain finding: rate limit bypass -- arbitrary number of requests bypass the cap (CWE-362, critical)
"""
from datetime import datetime, timedelta
from rate_limiter import get_db, get_counter, WINDOW_SECONDS, MAX_REQUESTS


def handle_api_request(user_id, payload):
    """
    Process a rate-limited API request for user_id.

    VULNERABLE: count is read and incremented in two separate statements.
    Concurrent callers all see the same stale count and all pass the limit check.
    """
    count, window_start = get_counter(user_id)

    now = datetime.utcnow()
    if window_start:
        ws = datetime.fromisoformat(window_start)
        if now - ws > timedelta(seconds=WINDOW_SECONDS):
            count = 0

    # TOCTOU: N concurrent requests can all read the same count < MAX_REQUESTS
    if count >= MAX_REQUESTS:
        return {"error": "Rate limit exceeded"}, 429

    conn = get_db()

# vuln-code-snippet start chain_rate_limit_race_vuln
    conn.execute(
        """
        INSERT INTO rate_counters (user_id, count, window_start)
        VALUES (?, 1, datetime('now'))
        ON CONFLICT(user_id) DO UPDATE SET count = count + 1
        """,
        (user_id,),
    )  # vuln-code-snippet vuln-line chain_rate_limit_race_vuln
# vuln-code-snippet end chain_rate_limit_race_vuln

    conn.commit()
    conn.close()

    return {"status": "ok", "echo": payload}, 200
