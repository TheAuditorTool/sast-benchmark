"""Inventory data storage for the order processing service.

This file is IDENTICAL between vuln/ and safe/ variants.
It provides stock lookup helpers; the race window is in orders.py
where the stock check and the decrement are separate operations.

Chain: concurrent POST /order -> SELECT stock -> UPDATE stock - quantity (if sufficient)
Individual findings: non-atomic stock check (medium)
Chain finding: oversell -- more items sold than in stock via TOCTOU race (CWE-362, critical)
"""
import sqlite3

DB_PATH = "inventory.db"


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


def init_db():
    conn = get_db()
    conn.execute("""
        CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            stock INTEGER NOT NULL DEFAULT 0
        )
    """)
    conn.execute("""
        CREATE TABLE IF NOT EXISTS orders (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            product_id INTEGER NOT NULL,
            quantity INTEGER NOT NULL,
            user_id INTEGER NOT NULL,
            created_at TEXT DEFAULT (datetime('now'))
        )
    """)
    conn.commit()
    conn.close()


def get_stock(product_id):
    """Return current stock for product_id, or None if not found."""
    conn = get_db()
    row = conn.execute(
        "SELECT stock FROM products WHERE id = ?", (product_id,)
    ).fetchone()
    conn.close()
    return row["stock"] if row else None
