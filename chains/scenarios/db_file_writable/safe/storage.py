"""Database file utilities -- SAFE variant.

Creates the SQLite database file with mode 0o600, restricting access to the
owning process only. No other local user can open the file directly with an
external SQLite client, preventing database record tampering.

Chain broken: DB file is owner-only -> no external SQLite access -> loader reads authentic records
"""
import os
import sqlite3

DB_PATH = "/tmp/app_data.db"


def init_db():
    """Create the database schema and set secure permissions."""
    conn = sqlite3.connect(DB_PATH)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users "
        "(id INTEGER PRIMARY KEY, username TEXT, role TEXT)"
    )
    conn.commit()
    conn.close()
    os.chmod(DB_PATH, 0o600)
