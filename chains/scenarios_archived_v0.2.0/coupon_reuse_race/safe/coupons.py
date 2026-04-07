"""Coupon storage and lookup for the e-commerce coupon service.

This file is IDENTICAL between vuln/ and safe/ variants.
It provides coupon creation and lookup; the race window is in redemption.py
where the used-flag check and the mark-used update are separate operations.

Chain: concurrent POST /redeem -> check used=0 (SELECT) -> mark used=1 (UPDATE)
Individual findings: non-atomic redemption check (medium)
Chain finding: single-use coupon redeemed multiple times via TOCTOU race (CWE-362, critical)
"""
import sqlite3

DB_PATH = "coupons.db"


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


def init_db():
    conn = get_db()
    conn.execute("""
        CREATE TABLE IF NOT EXISTS coupons (
            code TEXT PRIMARY KEY,
            discount_pct INTEGER NOT NULL,
            used INTEGER NOT NULL DEFAULT 0
        )
    """)
    conn.execute("""
        CREATE TABLE IF NOT EXISTS redemptions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            coupon_code TEXT NOT NULL,
            user_id INTEGER NOT NULL,
            redeemed_at TEXT DEFAULT (datetime('now'))
        )
    """)
    conn.commit()
    conn.close()


def get_coupon(code):
    """Return coupon row or None if not found."""
    conn = get_db()
    row = conn.execute(
        "SELECT code, discount_pct, used FROM coupons WHERE code = ?", (code,)
    ).fetchone()
    conn.close()
    return dict(row) if row else None
