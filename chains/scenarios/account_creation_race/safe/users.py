"""User storage for the registration service.

This file is IDENTICAL between vuln/ and safe/ variants.
It provides user lookup helpers; the race window is in registration.py
where the duplicate-email check and the INSERT are separate operations.

Chain: concurrent POST /register -> SELECT email check -> INSERT new user
Individual findings: non-atomic uniqueness check (medium)
Chain finding: duplicate accounts created via TOCTOU race (CWE-362, critical)
"""
import sqlite3

DB_PATH = "users.db"


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


def init_db():
    conn = get_db()
    conn.execute("""
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL,
            username TEXT NOT NULL,
            password_hash TEXT NOT NULL,
            created_at TEXT DEFAULT (datetime('now'))
        )
    """)
    conn.commit()
    conn.close()


def find_by_email(email):
    """Return user row for email, or None."""
    conn = get_db()
    row = conn.execute(
        "SELECT id, email, username FROM users WHERE email = ?", (email,)
    ).fetchone()
    conn.close()
    return dict(row) if row else None
