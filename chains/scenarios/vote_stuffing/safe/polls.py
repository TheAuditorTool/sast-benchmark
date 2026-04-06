"""Poll data storage for the voting service.

This file is IDENTICAL between vuln/ and safe/ variants.
It provides poll and option lookups; the race window is in voting.py
where the duplicate-vote check and the vote insert are separate operations.

Chain: concurrent POST /vote -> check existing vote (SELECT) -> insert vote (INSERT)
Individual findings: non-atomic duplicate check (medium)
Chain finding: one user casts multiple votes via TOCTOU race (CWE-362, critical)
"""
import sqlite3

DB_PATH = "polls.db"


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


def init_db():
    conn = get_db()
    conn.execute("""
        CREATE TABLE IF NOT EXISTS polls (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            question TEXT NOT NULL
        )
    """)
    conn.execute("""
        CREATE TABLE IF NOT EXISTS options (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            poll_id INTEGER NOT NULL,
            label TEXT NOT NULL
        )
    """)
    conn.execute("""
        CREATE TABLE IF NOT EXISTS votes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            poll_id INTEGER NOT NULL,
            option_id INTEGER NOT NULL,
            user_id INTEGER NOT NULL
        )
    """)
    conn.commit()
    conn.close()


def get_poll(poll_id):
    """Return poll row or None."""
    conn = get_db()
    row = conn.execute(
        "SELECT id, question FROM polls WHERE id = ?", (poll_id,)
    ).fetchone()
    conn.close()
    return dict(row) if row else None


def option_belongs_to_poll(option_id, poll_id):
    """Return True if option_id is a valid option for poll_id."""
    conn = get_db()
    row = conn.execute(
        "SELECT id FROM options WHERE id = ? AND poll_id = ?", (option_id, poll_id)
    ).fetchone()
    conn.close()
    return row is not None
