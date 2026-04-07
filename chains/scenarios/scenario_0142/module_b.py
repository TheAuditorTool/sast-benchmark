import sqlite3

DB_PATH = "app.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def get_profile_by_id(user_id):
    conn = get_db()
    cursor = conn.execute(
        "SELECT id, username, email, phone, address, bio, created_at FROM users WHERE id = ?",
        (user_id,),
    )
    row = cursor.fetchone()
    conn.close()
    return dict(row) if row else None

def get_all_users():
    conn = get_db()
    cursor = conn.execute("SELECT id, username FROM users WHERE active = 1")
    rows = cursor.fetchall()
    conn.close()
    return [dict(r) for r in rows]
