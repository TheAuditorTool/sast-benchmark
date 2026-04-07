"""Vote submission logic -- VULNERABLE variant.

Checks for an existing vote with a SELECT, then inserts the new vote with INSERT.
Two concurrent requests from the same user for the same poll both read zero existing
votes, both pass the uniqueness check, and both commit their INSERT -- stuffing the ballot.

Chain: concurrent POST /vote -> SELECT existing vote (none found) -> [race window] -> INSERT vote
Individual findings: non-atomic uniqueness check (medium)
Chain finding: vote stuffing -- one user casts multiple votes (CWE-362, critical)
"""
from polls import get_db, get_poll, option_belongs_to_poll


def cast_vote(poll_id, option_id, user_id):
    """
    Record a vote for option_id in poll_id from user_id.

    VULNERABLE: existing-vote check and the insert are two separate queries.
    Concurrent requests from the same user can both pass the check.
    """
    if not get_poll(poll_id):
        return {"error": "Poll not found"}, 404

    if not option_belongs_to_poll(option_id, poll_id):
        return {"error": "Invalid option for this poll"}, 400

    conn = get_db()
    existing = conn.execute(
        "SELECT id FROM votes WHERE poll_id = ? AND user_id = ?",
        (poll_id, user_id),
    ).fetchone()

    # TOCTOU: another concurrent request can pass this check before either inserts
    if existing:
        conn.close()
        return {"error": "Already voted"}, 409

# vuln-code-snippet start chain_vote_stuffing_vuln
    conn.execute(
        "INSERT INTO votes (poll_id, option_id, user_id) VALUES (?, ?, ?)",
        (poll_id, option_id, user_id),
    )  # vuln-code-snippet vuln-line chain_vote_stuffing_vuln
# vuln-code-snippet end chain_vote_stuffing_vuln

    conn.commit()
    conn.close()
    return {"status": "ok"}, 200
