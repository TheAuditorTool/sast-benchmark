import sqlite3

DB_PATH = "hotel.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def init_db():
    conn = get_db()
    conn.execute("""
        CREATE TABLE IF NOT EXISTS rooms (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            capacity INTEGER NOT NULL DEFAULT 1
        )
    """)
    conn.execute("""
        CREATE TABLE IF NOT EXISTS reservations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            room_id INTEGER NOT NULL,
            user_id INTEGER NOT NULL,
            check_in TEXT NOT NULL,
            check_out TEXT NOT NULL
        )
    """)
    conn.commit()
    conn.close()

def room_exists(room_id):
    conn = get_db()
    row = conn.execute(
        "SELECT id FROM rooms WHERE id = ?", (room_id,)
    ).fetchone()
    conn.close()
    return row is not None

def count_conflicts(room_id, check_in, check_out):
    conn = get_db()
    row = conn.execute(
        """
        SELECT COUNT(*) AS cnt FROM reservations
        WHERE room_id = ?
          AND check_in  < ?
          AND check_out > ?
        """,
        (room_id, check_out, check_in),
    ).fetchone()
    conn.close()
    return row["cnt"]
