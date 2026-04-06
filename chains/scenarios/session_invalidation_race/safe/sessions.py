"""Session store for the authentication service.

This file is IDENTICAL between vuln/ and safe/ variants.
It provides session creation, lookup, and invalidation; the race window is in auth.py
where session validity is cached at request start and re-used at the sensitive operation.

Chain: request start -> cache session valid -> logout invalidates -> cached check passes
Individual findings: stale session cache within request (medium)
Chain finding: logout does not prevent the in-flight request from completing (CWE-362, critical)
"""
import sqlite3
import secrets

DB_PATH = "sessions.db"


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


def init_db():
    conn = get_db()
    conn.execute("""
        CREATE TABLE IF NOT EXISTS sessions (
            token TEXT PRIMARY KEY,
            user_id INTEGER NOT NULL,
            valid INTEGER NOT NULL DEFAULT 1,
            created_at TEXT DEFAULT (datetime('now'))
        )
    """)
    conn.commit()
    conn.close()


def create_session(user_id):
    """Create a new valid session and return its token."""
    token = secrets.token_hex(32)
    conn = get_db()
    conn.execute(
        "INSERT INTO sessions (token, user_id, valid) VALUES (?, ?, 1)",
        (token, user_id),
    )
    conn.commit()
    conn.close()
    return token


def is_session_valid(token):
    """Return True if the session token exists and is still valid."""
    conn = get_db()
    row = conn.execute(
        "SELECT valid FROM sessions WHERE token = ?", (token,)
    ).fetchone()
    conn.close()
    return bool(row and row["valid"])


def invalidate_session(token):
    """Mark session invalid (logout)."""
    conn = get_db()
    conn.execute("UPDATE sessions SET valid = 0 WHERE token = ?", (token,))
    conn.commit()
    conn.close()
