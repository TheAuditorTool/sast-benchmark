import sqlite3
from polls import get_db, get_poll, option_belongs_to_poll

DB_PATH = "polls.db"

def _ensure_unique_constraint():
    conn = get_db()
    conn.execute("""
        CREATE UNIQUE INDEX IF NOT EXISTS uq_vote_per_user
        ON votes (poll_id, user_id)
    """)
    conn.commit()
    conn.close()

def cast_vote(poll_id, option_id, user_id):
    if not get_poll(poll_id):
        return {"error": "Poll not found"}, 404

    if not option_belongs_to_poll(option_id, poll_id):
        return {"error": "Invalid option for this poll"}, 400

    _ensure_unique_constraint()
    conn = get_db()

# vuln-code-snippet start ChainScenario0127B
    cursor = conn.execute(
        "INSERT OR IGNORE INTO votes (poll_id, option_id, user_id) VALUES (?, ?, ?)",
        (poll_id, option_id, user_id),
    )  # vuln-code-snippet target-line ChainScenario0127B
# vuln-code-snippet end ChainScenario0127B

    inserted = cursor.rowcount
    conn.commit()
    conn.close()

    if inserted == 0:
        return {"error": "Already voted"}, 409

    return {"status": "ok"}, 200
