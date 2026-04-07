import sqlite3

DB_PATH = "limits.db"
DAILY_LIMIT = 1000.0

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def init_db():
    conn = get_db()
    conn.execute("""
        CREATE TABLE IF NOT EXISTS daily_totals (
            user_id INTEGER NOT NULL,
            date TEXT NOT NULL,
            total REAL NOT NULL DEFAULT 0.0,
            PRIMARY KEY (user_id, date)
        )
    """)
    conn.execute("""
        CREATE TABLE IF NOT EXISTS withdrawal_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            amount REAL NOT NULL,
            created_at TEXT DEFAULT (datetime('now'))
        )
    """)
    conn.commit()
    conn.close()

def get_daily_total(user_id, date):
    conn = get_db()
    row = conn.execute(
        "SELECT total FROM daily_totals WHERE user_id = ? AND date = ?",
        (user_id, date),
    ).fetchone()
    conn.close()
    return row["total"] if row else 0.0
