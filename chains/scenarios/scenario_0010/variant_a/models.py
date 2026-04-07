import sqlite3

DB_PATH = "app.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def get_events_for_user(user_id, event_type=None, limit=100):
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
