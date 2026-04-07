import sqlite3
from accounts import get_db, get_balance, account_exists

def execute_transfer(from_owner, to_owner, amount):
    if amount <= 0:
        return {"error": "Amount must be positive"}, 400

    if not account_exists(from_owner):
        return {"error": "Source account not found"}, 404

    if not account_exists(to_owner):
        return {"error": "Destination account not found"}, 404

    balance = get_balance(from_owner)

    if balance < amount:
        return {"error": "Insufficient funds"}, 422

    conn = get_db()

# vuln-code-snippet start ChainScenario0118B
    conn.execute(
        "UPDATE accounts SET balance = balance - ? WHERE owner = ?",
        (amount, from_owner),
    )  # vuln-code-snippet target-line ChainScenario0118B
# vuln-code-snippet end ChainScenario0118B

    conn.execute(
        "UPDATE accounts SET balance = balance + ? WHERE owner = ?",
        (amount, to_owner),
    )
    conn.execute(
        "INSERT INTO transfers (from_owner, to_owner, amount) VALUES (?, ?, ?)",
        (from_owner, to_owner, amount),
    )
    conn.commit()
    conn.close()

    new_balance = get_balance(from_owner)
    return {"status": "ok", "new_balance": new_balance}, 200
