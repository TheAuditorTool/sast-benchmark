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
    conn = get_db()
    row = conn.execute(
        "SELECT code, discount_pct, used FROM coupons WHERE code = ?", (code,)
    ).fetchone()
    conn.close()
    return dict(row) if row else None
