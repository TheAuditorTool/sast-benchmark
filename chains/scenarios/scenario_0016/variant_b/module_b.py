import sqlite3

DB_PATH = "app.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def get_user_in_org(org_id, user_id):
    conn = get_db()
    cursor = conn.execute(
        "SELECT u.id, u.username, u.email, u.role, u.phone, u.created_at "
        "FROM users u JOIN org_members m ON m.user_id = u.id "
        "WHERE m.org_id = ? AND u.id = ?",
        (org_id, user_id),
    )
    row = cursor.fetchone()
    conn.close()
    return dict(row) if row else None

def is_org_member(org_id, user_id):
    conn = get_db()
    cursor = conn.execute(
        "SELECT 1 FROM org_members WHERE org_id = ? AND user_id = ?",
        (org_id, user_id),
    )
    row = cursor.fetchone()
    conn.close()
    return row is not None

def list_org_members(org_id):
    conn = get_db()
    cursor = conn.execute(
        "SELECT u.id, u.username, u.role FROM users u "
        "JOIN org_members m ON m.user_id = u.id WHERE m.org_id = ?",
        (org_id,),
    )
    rows = cursor.fetchall()
    conn.close()
    return [dict(r) for r in rows]
