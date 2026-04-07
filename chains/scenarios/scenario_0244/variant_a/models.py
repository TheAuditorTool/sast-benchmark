import sqlite3
import json

DB_PATH = "app.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def get_event(event_id):
    conn = get_db()
    cursor = conn.execute(
        "SELECT id, tenant_id, event_type, user_id, payload, occurred_at FROM events WHERE id = ?",
        (event_id,),
    )
    row = cursor.fetchone()
    conn.close()
    if row is None:
        return None
    record = dict(row)
    if record.get("payload"):
        record["payload"] = json.loads(record["payload"])
    return record

def get_webhook_endpoints_for_tenant(tenant_id):
    conn = get_db()
    cursor = conn.execute(
        "SELECT id, url, secret FROM webhooks WHERE tenant_id = ? AND active = 1",
        (tenant_id,),
    )
    rows = cursor.fetchall()
    conn.close()
    return [dict(r) for r in rows]

def get_all_webhook_endpoints():
    conn = get_db()
    cursor = conn.execute(
        "SELECT id, tenant_id, url, secret FROM webhooks WHERE active = 1"
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
