"""Database file utilities -- VULNERABLE variant.

Creates the SQLite database file with mode 0o666, making it world-writable.
Any local user can open and modify the database directly with the sqlite3
CLI, injecting, updating, or deleting records that the application then
serves as authoritative data.

Chain: world-writable DB file created -> attacker modifies DB directly -> loader serves tampered data
Individual findings: insecure database file permissions (CWE-732)
Chain finding: data tampering via direct database file write (critical)
"""
import os
import sqlite3

DB_PATH = "/tmp/app_data.db"


def init_db():
    """Create the database schema and set insecure permissions."""
    conn = sqlite3.connect(DB_PATH)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users "
        "(id INTEGER PRIMARY KEY, username TEXT, role TEXT)"
    )
    conn.commit()
    conn.close()
    os.chmod(DB_PATH, 0o666)
