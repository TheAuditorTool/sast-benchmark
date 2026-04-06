"""Data layer for user profiles.

This file is IDENTICAL between vuln/ and safe/ variants.
Provides database access for user profile records stored in a
sequential-integer-keyed users table, simulating a typical SaaS
user directory.
"""
import sqlite3

DB_PATH = "app.db"


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


def get_profile_by_id(user_id):
    """Return full profile row for the given user_id, or None."""
    conn = get_db()
    cursor = conn.execute(
        "SELECT id, username, email, phone, address, bio, created_at FROM users WHERE id = ?",
        (user_id,),
    )
    row = cursor.fetchone()
    conn.close()
    return dict(row) if row else None


def get_all_users():
    """Return minimal list of all active user records."""
    conn = get_db()
    cursor = conn.execute("SELECT id, username FROM users WHERE active = 1")
    rows = cursor.fetchall()
    conn.close()
    return [dict(r) for r in rows]
