"""Vote submission logic -- SAFE variant.

Creates a UNIQUE constraint on (poll_id, user_id) and uses INSERT OR IGNORE.
The database engine enforces uniqueness atomically, so concurrent inserts from
the same user are serialized -- only one will succeed.

Chain: concurrent POST /vote -> INSERT OR IGNORE (DB enforces UNIQUE constraint)
Individual findings: none -- uniqueness is enforced at the database level
Chain finding: none -- vote stuffing is prevented by the constraint (CWE-362 mitigated)
"""
import sqlite3
from polls import get_db, get_poll, option_belongs_to_poll

DB_PATH = "polls.db"


def _ensure_unique_constraint():
    """Create the unique index if it was not created during init_db."""
    conn = get_db()
    conn.execute("""
        CREATE UNIQUE INDEX IF NOT EXISTS uq_vote_per_user
        ON votes (poll_id, user_id)
    """)
    conn.commit()
    conn.close()


def cast_vote(poll_id, option_id, user_id):
    """
    Record a vote for option_id in poll_id from user_id.

    SAFE: INSERT OR IGNORE relies on the UNIQUE index (poll_id, user_id).
    Concurrent inserts from the same user are handled atomically by SQLite.
    """
    if not get_poll(poll_id):
        return {"error": "Poll not found"}, 404

    if not option_belongs_to_poll(option_id, poll_id):
        return {"error": "Invalid option for this poll"}, 400

    _ensure_unique_constraint()
    conn = get_db()

# vuln-code-snippet start chain_vote_stuffing_safe
    cursor = conn.execute(
        "INSERT OR IGNORE INTO votes (poll_id, option_id, user_id) VALUES (?, ?, ?)",
        (poll_id, option_id, user_id),
    )  # vuln-code-snippet safe-line chain_vote_stuffing_safe
# vuln-code-snippet end chain_vote_stuffing_safe

    inserted = cursor.rowcount
    conn.commit()
    conn.close()

    if inserted == 0:
        return {"error": "Already voted"}, 409

    return {"status": "ok"}, 200
