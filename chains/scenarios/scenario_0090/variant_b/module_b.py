import sqlite3

DB_PATH = "app.db"
PAGE_SIZE = 25

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def get_customers_page(tenant_id, after_id=0):
    conn = get_db()
    cursor = conn.execute(
        "SELECT id, tenant_id, name, email, plan, mrr_cents FROM customers "
        "WHERE tenant_id = ? AND id > ? ORDER BY id LIMIT ?",
        (tenant_id, after_id, PAGE_SIZE),
    )
    rows = cursor.fetchall()
    conn.close()
    return [dict(r) for r in rows]

def get_customers_page_no_tenant_check(after_id=0):
    conn = get_db()
    cursor = conn.execute(
        "SELECT id, tenant_id, name, email, plan, mrr_cents FROM customers "
        "WHERE id > ? ORDER BY id LIMIT ?",
        (after_id, PAGE_SIZE),
    )
    rows = cursor.fetchall()
    conn.close()
    return [dict(r) for r in rows]

def get_tenant_for_user(user_id):
    conn = get_db()
    cursor = conn.execute("SELECT tenant_id FROM users WHERE id = ?", (user_id,))
    row = cursor.fetchone()
    conn.close()
    return row["tenant_id"] if row else None
