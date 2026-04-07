"""Reservation logic -- VULNERABLE variant.

Queries for conflicting reservations with SELECT COUNT(*), then inserts the new
reservation in a separate statement.  Two concurrent requests for the same room
and overlapping dates both find zero conflicts and both successfully insert,
resulting in a double-booking.

Chain: concurrent POST /book -> SELECT conflicts (0) -> [race window] -> INSERT reservation
Individual findings: non-atomic availability check (medium)
Chain finding: double-booking via TOCTOU race (CWE-362, critical)
"""
from rooms import get_db, room_exists, count_conflicts


def create_booking(room_id, user_id, check_in, check_out):
    """
    Reserve room_id for user_id from check_in to check_out.

    VULNERABLE: conflict check and insert are two separate queries.
    Concurrent bookings for the same room can both pass the conflict check.
    """
    if not room_exists(room_id):
        return {"error": "Room not found"}, 404

    if check_in >= check_out:
        return {"error": "check_out must be after check_in"}, 400

    # TOCTOU: another request can pass this check before either inserts
    if count_conflicts(room_id, check_in, check_out) > 0:
        return {"error": "Room not available for requested dates"}, 409

    conn = get_db()

# vuln-code-snippet start chain_reservation_race_vuln
    conn.execute(
        "INSERT INTO reservations (room_id, user_id, check_in, check_out) VALUES (?, ?, ?, ?)",
        (room_id, user_id, check_in, check_out),
    )  # vuln-code-snippet vuln-line chain_reservation_race_vuln
# vuln-code-snippet end chain_reservation_race_vuln

    conn.commit()
    conn.close()
    return {"status": "ok", "room_id": room_id, "check_in": check_in, "check_out": check_out}, 201
