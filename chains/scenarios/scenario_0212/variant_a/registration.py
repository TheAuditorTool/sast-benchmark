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
    if not email or not username or not password:
        return {"error": "email, username, and password are required"}, 400

    _ensure_unique_email()
    password_hash = _hash_password(password)
    conn = get_db()

# vuln-code-snippet start ChainScenario0212A
    try:
        conn.execute(
            "INSERT INTO users (email, username, password_hash) VALUES (?, ?, ?)",
            (email, username, password_hash),
        )  # vuln-code-snippet target-line ChainScenario0212A
        conn.commit()
    except sqlite3.IntegrityError:
        conn.close()
        return {"error": "Email already registered"}, 409
# vuln-code-snippet end ChainScenario0212A

    conn.close()
    return {"status": "ok", "email": email}, 201
