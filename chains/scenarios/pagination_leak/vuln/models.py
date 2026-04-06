"""Data layer for the SaaS multi-tenant customer list.

This file is IDENTICAL between vuln/ and safe/ variants.
Records are scoped to tenants.  The pagination cursor is a plain
integer (last seen row id), which is predictable across tenants.
An attacker on tenant A who knows the row id range for tenant B can
produce a cursor that causes the next-page query to return tenant B's
records.
"""
import sqlite3

DB_PATH = "app.db"
PAGE_SIZE = 25


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


def get_customers_page(tenant_id, after_id=0):
    """Return the next page of customers for tenant_id after row after_id."""
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
    """Return the next page of ALL customers regardless of tenant."""
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
    """Return the tenant_id for the given user."""
    conn = get_db()
    cursor = conn.execute("SELECT tenant_id FROM users WHERE id = ?", (user_id,))
    row = cursor.fetchone()
    conn.close()
    return row["tenant_id"] if row else None
