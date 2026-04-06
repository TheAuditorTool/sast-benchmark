"""User registration logic -- SAFE variant.

Adds a UNIQUE constraint on the email column and catches the IntegrityError
raised by SQLite when a duplicate insert is attempted.  The constraint is
enforced atomically by the database engine -- no separate existence check needed.

Chain: concurrent POST /register -> INSERT (UNIQUE email constraint) -> IntegrityError on dup
Individual findings: none -- uniqueness enforced at the database level
Chain finding: none -- duplicate accounts are impossible (CWE-362 mitigated)
"""
import hashlib
import sqlite3
from users import get_db


def _hash_password(password):
    return hashlib.sha256(password.encode()).hexdigest()


def _ensure_unique_email():
    conn = get_db()
    conn.execute("CREATE UNIQUE INDEX IF NOT EXISTS uq_email ON users (email)")
    conn.commit()
    conn.close()


def register_user(email, username, password):
    """
    Create a new user account.

    SAFE: UNIQUE index on email means the database rejects duplicate inserts
    atomically.  No separate SELECT check required.
    """
    if not email or not username or not password:
        return {"error": "email, username, and password are required"}, 400

    _ensure_unique_email()
    password_hash = _hash_password(password)
    conn = get_db()

# vuln-code-snippet start chain_account_race_safe
    try:
        conn.execute(
            "INSERT INTO users (email, username, password_hash) VALUES (?, ?, ?)",
            (email, username, password_hash),
        )  # vuln-code-snippet safe-line chain_account_race_safe
        conn.commit()
    except sqlite3.IntegrityError:
        conn.close()
        return {"error": "Email already registered"}, 409
# vuln-code-snippet end chain_account_race_safe

    conn.close()
    return {"status": "ok", "email": email}, 201
