from rooms import get_db, room_exists, count_conflicts

def create_booking(room_id, user_id, check_in, check_out):
    if not room_exists(room_id):
        return {"error": "Room not found"}, 404

    if check_in >= check_out:
        return {"error": "check_out must be after check_in"}, 400

    if count_conflicts(room_id, check_in, check_out) > 0:
        return {"error": "Room not available for requested dates"}, 409

    conn = get_db()

# vuln-code-snippet start ChainScenario0031B
    conn.execute(
        "INSERT INTO reservations (room_id, user_id, check_in, check_out) VALUES (?, ?, ?, ?)",
        (room_id, user_id, check_in, check_out),
    )  # vuln-code-snippet target-line ChainScenario0031B
# vuln-code-snippet end ChainScenario0031B

    conn.commit()
    conn.close()
    return {"status": "ok", "room_id": room_id, "check_in": check_in, "check_out": check_out}, 201
