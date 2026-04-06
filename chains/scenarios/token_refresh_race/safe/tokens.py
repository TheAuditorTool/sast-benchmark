"""Token store for the authentication token refresh service.

This file is IDENTICAL between vuln/ and safe/ variants.
It provides token creation, validation, and invalidation; the race window
is in refresh.py where the old token is validated, then a new one is issued,
in separate non-atomic operations.

Chain: concurrent POST /refresh -> validate old token -> issue new token (TOCTOU replay)
Individual findings: non-atomic token rotation (medium)
Chain finding: old token replayed multiple times to obtain multiple valid tokens (CWE-362, critical)
"""
import sqlite3
import secrets
from datetime import datetime, timedelta

DB_PATH = "tokens.db"
TOKEN_TTL_SECONDS = 3600


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


def init_db():
    conn = get_db()
    conn.execute("""
        CREATE TABLE IF NOT EXISTS tokens (
            token TEXT PRIMARY KEY,
            user_id INTEGER NOT NULL,
            valid INTEGER NOT NULL DEFAULT 1,
            expires_at TEXT NOT NULL
        )
    """)
    conn.commit()
    conn.close()


def create_token(user_id):
    """Create and persist a new valid token for user_id."""
    token = secrets.token_hex(32)
    expires_at = (datetime.utcnow() + timedelta(seconds=TOKEN_TTL_SECONDS)).isoformat()
    conn = get_db()
    conn.execute(
        "INSERT INTO tokens (token, user_id, valid, expires_at) VALUES (?, ?, 1, ?)",
        (token, user_id, expires_at),
    )
    conn.commit()
    conn.close()
    return token


def get_valid_token(token):
    """Return token row if it exists, is valid, and has not expired. Else None."""
    conn = get_db()
    row = conn.execute(
        "SELECT token, user_id, valid, expires_at FROM tokens WHERE token = ?",
        (token,),
    ).fetchone()
    conn.close()
    if not row:
        return None
    if not row["valid"]:
        return None
    if datetime.fromisoformat(row["expires_at"]) < datetime.utcnow():
        return None
    return dict(row)


def invalidate_token(token):
    """Mark token as invalid (consumed)."""
    conn = get_db()
    conn.execute("UPDATE tokens SET valid = 0 WHERE token = ?", (token,))
    conn.commit()
    conn.close()
