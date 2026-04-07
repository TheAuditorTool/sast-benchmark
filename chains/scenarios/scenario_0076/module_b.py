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
    conn = get_db()
    row = conn.execute(
        "SELECT id, question FROM polls WHERE id = ?", (poll_id,)
    ).fetchone()
    conn.close()
    return dict(row) if row else None

def option_belongs_to_poll(option_id, poll_id):
    conn = get_db()
    row = conn.execute(
        "SELECT id FROM options WHERE id = ? AND poll_id = ?", (option_id, poll_id)
    ).fetchone()
    conn.close()
    return row is not None
