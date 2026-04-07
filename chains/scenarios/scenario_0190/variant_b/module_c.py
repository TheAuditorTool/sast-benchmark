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
    conn = get_db()
    row = conn.execute(
        "SELECT valid FROM sessions WHERE token = ?", (token,)
    ).fetchone()
    conn.close()
    return bool(row and row["valid"])

def invalidate_session(token):
    conn = get_db()
    conn.execute("UPDATE sessions SET valid = 0 WHERE token = ?", (token,))
    conn.commit()
    conn.close()
