"""User registration logic -- VULNERABLE variant.

Checks for an existing email with SELECT, then inserts the new user with INSERT.
Two concurrent registration requests for the same email both find no existing user,
both pass the uniqueness check, and both commit their INSERT -- creating duplicates.

Chain: concurrent POST /register -> SELECT email (none found) -> [race window] -> INSERT user
Individual findings: non-atomic email uniqueness check (medium)
Chain finding: duplicate accounts for the same email (CWE-362, critical)
"""
import hashlib
from users import get_db, find_by_email


def _hash_password(password):
    return hashlib.sha256(password.encode()).hexdigest()


def register_user(email, username, password):
    """
    Create a new user account.

    VULNERABLE: email existence check and INSERT are separate queries.
    Concurrent requests can both pass the check before either inserts.
    """
    if not email or not username or not password:
        return {"error": "email, username, and password are required"}, 400

    existing = find_by_email(email)

    # TOCTOU: another request can pass this check before either inserts
    if existing:
        return {"error": "Email already registered"}, 409

    password_hash = _hash_password(password)
    conn = get_db()

# vuln-code-snippet start chain_account_race_vuln
    conn.execute(
        "INSERT INTO users (email, username, password_hash) VALUES (?, ?, ?)",
        (email, username, password_hash),
    )  # vuln-code-snippet vuln-line chain_account_race_vuln
# vuln-code-snippet end chain_account_race_vuln

    conn.commit()
    conn.close()
    return {"status": "ok", "email": email}, 201
