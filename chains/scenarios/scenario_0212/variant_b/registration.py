import hashlib
from users import get_db, find_by_email

def _hash_password(password):
    return hashlib.sha256(password.encode()).hexdigest()

def register_user(email, username, password):
    if not email or not username or not password:
        return {"error": "email, username, and password are required"}, 400

    existing = find_by_email(email)

    if existing:
        return {"error": "Email already registered"}, 409

    password_hash = _hash_password(password)
    conn = get_db()

# vuln-code-snippet start ChainScenario0212B
    conn.execute(
        "INSERT INTO users (email, username, password_hash) VALUES (?, ?, ?)",
        (email, username, password_hash),
    )  # vuln-code-snippet target-line ChainScenario0212B
# vuln-code-snippet end ChainScenario0212B

    conn.commit()
    conn.close()
    return {"status": "ok", "email": email}, 201
