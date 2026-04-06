"""Data layer for the analytics reporting service.

This file is IDENTICAL between vuln/ and safe/ variants.
Events are scoped to individual users.  The API exposes a filter
endpoint that is intended to return only the caller's own events
but accepts a user_id query parameter for the WHERE clause.
"""
import sqlite3

DB_PATH = "app.db"


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


def get_events_for_user(user_id, event_type=None, limit=100):
    """Return events for the given user_id, optionally filtered by type."""
    conn = get_db()
    if event_type:
        cursor = conn.execute(
            "SELECT id, user_id, event_type, payload, occurred_at "
            "FROM events WHERE user_id = ? AND event_type = ? ORDER BY occurred_at DESC LIMIT ?",
            (user_id, event_type, limit),
        )
    else:
        cursor = conn.execute(
            "SELECT id, user_id, event_type, payload, occurred_at "
            "FROM events WHERE user_id = ? ORDER BY occurred_at DESC LIMIT ?",
            (user_id, limit),
        )
    rows = cursor.fetchall()
    conn.close()
    return [dict(r) for r in rows]
