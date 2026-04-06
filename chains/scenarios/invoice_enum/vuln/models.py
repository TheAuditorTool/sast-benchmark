"""Data layer for the billing service.

This file is IDENTICAL between vuln/ and safe/ variants.
Invoices are sequential integers scoped to a tenant.  The tenant_id
column enforces multi-tenancy; the amount_cents and line_items columns
contain commercially sensitive billing data.
"""
import sqlite3
import json

DB_PATH = "app.db"


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


def get_invoice(invoice_id):
    """Return full invoice row for the given id, or None."""
    conn = get_db()
    cursor = conn.execute(
        "SELECT id, tenant_id, amount_cents, status, due_date, line_items, created_at "
        "FROM invoices WHERE id = ?",
        (invoice_id,),
    )
    row = cursor.fetchone()
    conn.close()
    if row is None:
        return None
    record = dict(row)
    if record.get("line_items"):
        record["line_items"] = json.loads(record["line_items"])
    return record


def list_tenant_invoices(tenant_id):
    """Return all invoices for the given tenant."""
    conn = get_db()
    cursor = conn.execute(
        "SELECT id, amount_cents, status, due_date FROM invoices WHERE tenant_id = ?",
        (tenant_id,),
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
