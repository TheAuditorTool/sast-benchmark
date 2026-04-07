import sqlite3

DB_PATH = "app.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def get_contacts_for_user(user_id):
    conn = get_db()
    cursor = conn.execute(
        "SELECT id, display_name, email, target_user_uuid FROM contacts WHERE owner_id = ?",
        (user_id,),
    )
    rows = cursor.fetchall()
    conn.close()
    return [dict(r) for r in rows]

def get_user_by_uuid(uuid):
    conn = get_db()
    cursor = conn.execute(
        "SELECT id, username, email, phone, ssn_last4, plan FROM users WHERE uuid = ?",
        (uuid,),
    )
    row = cursor.fetchone()
    conn.close()
    return dict(row) if row else None

def get_user_uuid(user_id):
    conn = get_db()
    cursor = conn.execute("SELECT uuid FROM users WHERE id = ?", (user_id,))
    row = cursor.fetchone()
    conn.close()
    return row["uuid"] if row else None
