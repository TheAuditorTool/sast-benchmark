import sqlite3
import json

DB_PATH = "app.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def _fetch_all(cursor):
    return [dict(r) for r in cursor.fetchall()]

def build_user_export(user_id):
    conn = get_db()
    orders = _fetch_all(conn.execute(
        "SELECT id, total_cents, status, created_at FROM orders WHERE user_id = ?",
        (user_id,),
    ))
    addresses = _fetch_all(conn.execute(
        "SELECT id, line1, line2, city, country, postcode FROM addresses WHERE user_id = ?",
        (user_id,),
    ))
    tickets = _fetch_all(conn.execute(
        "SELECT id, subject, status, created_at FROM support_tickets WHERE user_id = ?",
        (user_id,),
    ))
    conn.close()
    return {"user_id": user_id, "orders": orders,
            "addresses": addresses, "support_tickets": tickets}
