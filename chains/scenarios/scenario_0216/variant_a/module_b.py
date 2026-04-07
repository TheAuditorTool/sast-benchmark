import sqlite3

DB_PATH = "app.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def get_thread_messages(thread_id):
    conn = get_db()
    cursor = conn.execute(
        "SELECT id, sender_id, body, sent_at FROM messages WHERE thread_id = ? ORDER BY sent_at",
        (thread_id,),
    )
    rows = cursor.fetchall()
    conn.close()
    return [dict(r) for r in rows]

def is_thread_participant(thread_id, user_id):
    conn = get_db()
    cursor = conn.execute(
        "SELECT 1 FROM thread_participants WHERE thread_id = ? AND user_id = ?",
        (thread_id, user_id),
    )
    row = cursor.fetchone()
    conn.close()
    return row is not None

def get_thread_metadata(thread_id):
    conn = get_db()
    cursor = conn.execute(
        "SELECT id, subject, created_at FROM threads WHERE id = ?",
        (thread_id,),
    )
    row = cursor.fetchone()
    conn.close()
    return dict(row) if row else None
