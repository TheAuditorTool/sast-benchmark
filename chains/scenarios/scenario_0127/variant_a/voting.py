from polls import get_db, get_poll, option_belongs_to_poll

def cast_vote(poll_id, option_id, user_id):
    if not get_poll(poll_id):
        return {"error": "Poll not found"}, 404

    if not option_belongs_to_poll(option_id, poll_id):
        return {"error": "Invalid option for this poll"}, 400

    conn = get_db()
    existing = conn.execute(
        "SELECT id FROM votes WHERE poll_id = ? AND user_id = ?",
        (poll_id, user_id),
    ).fetchone()

    if existing:
        conn.close()
        return {"error": "Already voted"}, 409

# vuln-code-snippet start ChainScenario0127A
    conn.execute(
        "INSERT INTO votes (poll_id, option_id, user_id) VALUES (?, ?, ?)",
        (poll_id, option_id, user_id),
    )  # vuln-code-snippet target-line ChainScenario0127A
# vuln-code-snippet end ChainScenario0127A

    conn.commit()
    conn.close()
    return {"status": "ok"}, 200
