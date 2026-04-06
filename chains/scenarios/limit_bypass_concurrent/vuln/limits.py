"""Daily withdrawal limit tracking for the banking service.

This file is IDENTICAL between vuln/ and safe/ variants.
It provides daily total lookups and tracking; the race window is in withdrawals.py
where the daily total check and the withdrawal update are separate operations.

Chain: concurrent POST /withdraw -> SELECT daily total -> UPDATE total + amount (if under limit)
Individual findings: non-atomic limit check (medium)
Chain finding: daily limit exceeded via concurrent withdrawals (CWE-362, critical)
"""
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
    """Return the total withdrawn by user_id on date, or 0.0 if no record."""
    conn = get_db()
    row = conn.execute(
        "SELECT total FROM daily_totals WHERE user_id = ? AND date = ?",
        (user_id, date),
    ).fetchone()
    conn.close()
    return row["total"] if row else 0.0
