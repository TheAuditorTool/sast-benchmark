import sqlite3

DB_PATH = "ratelimit.db"
WINDOW_SECONDS = 60
MAX_REQUESTS = 10

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def init_db():
    conn = get_db()
    conn.execute("""
        CREATE TABLE IF NOT EXISTS rate_counters (
            user_id INTEGER PRIMARY KEY,
            count INTEGER NOT NULL DEFAULT 0,
            window_start TEXT NOT NULL DEFAULT (datetime('now'))
        )
    """)
    conn.commit()
    conn.close()

def get_counter(user_id):
    conn = get_db()
    row = conn.execute(
        "SELECT count, window_start FROM rate_counters WHERE user_id = ?",
        (user_id,),
    ).fetchone()
    conn.close()
    if row:
        return row["count"], row["window_start"]
    return 0, None
