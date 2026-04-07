import sqlite3

DB_PATH = "permissions.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def init_db():
    conn = get_db()
    conn.execute("""
        CREATE TABLE IF NOT EXISTS user_permissions (
            user_id INTEGER NOT NULL,
            permission TEXT NOT NULL,
            PRIMARY KEY (user_id, permission)
        )
    """)
    conn.commit()
    conn.close()

def has_permission(user_id, permission):
    conn = get_db()
    row = conn.execute(
        "SELECT 1 FROM user_permissions WHERE user_id = ? AND permission = ?",
        (user_id, permission),
    ).fetchone()
    conn.close()
    return row is not None

def grant_permission(user_id, permission):
    conn = get_db()
    conn.execute(
        "INSERT OR IGNORE INTO user_permissions (user_id, permission) VALUES (?, ?)",
        (user_id, permission),
    )
    conn.commit()
    conn.close()

def revoke_permission(user_id, permission):
    conn = get_db()
    conn.execute(
        "DELETE FROM user_permissions WHERE user_id = ? AND permission = ?",
        (user_id, permission),
    )
    conn.commit()
    conn.close()
