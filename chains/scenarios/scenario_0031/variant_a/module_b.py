import sqlite3
from module_c import get_db, room_exists

DB_PATH = "hotel.db"

def create_booking(room_id, user_id, check_in, check_out):
    if not room_exists(room_id):
        return {"error": "Room not found"}, 404

    if check_in >= check_out:
        return {"error": "check_out must be after check_in"}, 400

    conn = get_db()

    conn.execute("BEGIN IMMEDIATE")
    conflicts = conn.execute(
        """
        SELECT COUNT(*) AS cnt FROM reservations
        WHERE room_id = ?
          AND check_in  < ?
          AND check_out > ?
        """,
        (room_id, check_out, check_in),
    ).fetchone()["cnt"]

    if conflicts > 0:
        conn.execute("ROLLBACK")
        conn.close()
        return {"error": "Room not available for requested dates"}, 409

    conn.execute(
        "INSERT INTO reservations (room_id, user_id, check_in, check_out) VALUES (?, ?, ?, ?)",
        (room_id, user_id, check_in, check_out),
    )
    conn.execute("COMMIT")

    conn.close()
    return {"status": "ok", "room_id": room_id, "check_in": check_in, "check_out": check_out}, 201
