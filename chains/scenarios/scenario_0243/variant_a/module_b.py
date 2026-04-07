import sqlite3
import base64

DB_PATH = "app.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def decode_node_id(node_id):
    try:
        decoded = base64.b64decode(node_id).decode("utf-8")
        parts = decoded.split(":", 1)
        if len(parts) != 2:
            return None, None
        return parts[0], int(parts[1])
    except Exception:
        return None, None

def fetch_node(type_name, pk):
    conn = get_db()
    if type_name == "Order":
        cursor = conn.execute(
            "SELECT id, user_id, total_cents, status, created_at FROM orders WHERE id = ?",
            (pk,),
        )
    elif type_name == "Address":
        cursor = conn.execute(
            "SELECT id, user_id, line1, city, country FROM addresses WHERE id = ?",
            (pk,),
        )
    elif type_name == "UserProfile":
        cursor = conn.execute(
            "SELECT id, username, email, phone, bio FROM users WHERE id = ?",
            (pk,),
        )
    else:
        conn.close()
        return None
    row = cursor.fetchone()
    conn.close()
    if row is None:
        return None
    record = dict(row)
    record["__type"] = type_name
    return record

def get_owner_for_node(type_name, pk):
    conn = get_db()
    if type_name in ("Order", "Address"):
        cursor = conn.execute(
            "SELECT user_id FROM " + ("orders" if type_name == "Order" else "addresses") + " WHERE id = ?",
            (pk,),
        )
        row = cursor.fetchone()
        conn.close()
        return row["user_id"] if row else None
    if type_name == "UserProfile":
        conn.close()
        return pk
    conn.close()
    return None
