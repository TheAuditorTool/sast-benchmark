import sqlite3

DB_PATH = "bank.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def init_db():
    conn = get_db()
    conn.execute("""
        CREATE TABLE IF NOT EXISTS accounts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            owner TEXT NOT NULL UNIQUE,
            balance REAL NOT NULL DEFAULT 0.0
        )
    """)
    conn.execute("""
        CREATE TABLE IF NOT EXISTS transfers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            from_owner TEXT NOT NULL,
            to_owner TEXT NOT NULL,
            amount REAL NOT NULL,
            created_at TEXT DEFAULT (datetime('now'))
        )
    """)
    conn.commit()
    conn.close()

def get_balance(owner):
    conn = get_db()
    row = conn.execute(
        "SELECT balance FROM accounts WHERE owner = ?", (owner,)
    ).fetchone()
    conn.close()
    return row["balance"] if row else None

def account_exists(owner):
    conn = get_db()
    row = conn.execute(
        "SELECT id FROM accounts WHERE owner = ?", (owner,)
    ).fetchone()
    conn.close()
    return row is not None
