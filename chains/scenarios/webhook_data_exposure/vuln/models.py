"""Data layer for the webhook delivery service.

This file is IDENTICAL between vuln/ and safe/ variants.
Webhooks are registered by tenants to receive event notifications.
The delivery payload is built from the full event record, which
includes columns that are only relevant to the tenant that owns the
event.  Webhook endpoints are stored per-tenant; a misconfigured
delivery loop can send one tenant's full payload to another tenant's
endpoint.
"""
import sqlite3
import json

DB_PATH = "app.db"


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


def get_event(event_id):
    """Return the full event row for the given id."""
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
    """Return all active webhook endpoint URLs for a tenant."""
    conn = get_db()
    cursor = conn.execute(
        "SELECT id, url, secret FROM webhooks WHERE tenant_id = ? AND active = 1",
        (tenant_id,),
    )
    rows = cursor.fetchall()
    conn.close()
    return [dict(r) for r in rows]


def get_all_webhook_endpoints():
    """Return all active webhook endpoints regardless of tenant."""
    conn = get_db()
    cursor = conn.execute(
        "SELECT id, tenant_id, url, secret FROM webhooks WHERE active = 1"
    )
    rows = cursor.fetchall()
    conn.close()
    return [dict(r) for r in rows]


def get_tenant_for_user(user_id):
    """Return the tenant_id for the given user."""
    conn = get_db()
    cursor = conn.execute("SELECT tenant_id FROM users WHERE id = ?", (user_id,))
    row = cursor.fetchone()
    conn.close()
    return row["tenant_id"] if row else None
