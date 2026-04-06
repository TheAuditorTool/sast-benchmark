"""Reservation logic -- SAFE variant.

Wraps the conflict check and the insert in a single IMMEDIATE transaction,
which acquires a write lock before reading.  Concurrent transactions are
serialized: the second transaction observes the first's insert before
deciding whether to proceed.

Chain: concurrent POST /book -> BEGIN IMMEDIATE -> SELECT conflicts -> INSERT (serialized)
Individual findings: none -- conflict check and insert are within a serialized transaction
Chain finding: none -- double-booking is impossible under IMMEDIATE isolation (CWE-362 mitigated)
"""
import sqlite3
from rooms import get_db, room_exists

DB_PATH = "hotel.db"


def create_booking(room_id, user_id, check_in, check_out):
    """
    Reserve room_id for user_id from check_in to check_out.

    SAFE: BEGIN IMMEDIATE acquires a reserved lock before the conflict check,
    so concurrent transactions are serialized at the lock boundary.
    """
    if not room_exists(room_id):
        return {"error": "Room not found"}, 404

    if check_in >= check_out:
        return {"error": "check_out must be after check_in"}, 400

    conn = get_db()

# vuln-code-snippet start chain_reservation_race_safe
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
    )  # vuln-code-snippet safe-line chain_reservation_race_safe
    conn.execute("COMMIT")
# vuln-code-snippet end chain_reservation_race_safe

    conn.close()
    return {"status": "ok", "room_id": room_id, "check_in": check_in, "check_out": check_out}, 201
