"""Token refresh logic -- SAFE variant.

Uses an atomic UPDATE WHERE valid=1 to invalidate the old token.  If rowcount
is 0, the token was already consumed by a concurrent refresh.  The new token
is only issued after the atomic invalidation succeeds, so only one refresh
request can exchange any given old token.

Chain: concurrent POST /refresh -> atomic UPDATE valid=0 WHERE valid=1 -> rowcount check -> issue new
Individual findings: none -- invalidation and issuance are sequenced correctly
Chain finding: none -- replay is impossible because only one caller sees rowcount=1 (CWE-362 mitigated)
"""
from tokens import get_db, get_valid_token, create_token


def refresh_token(old_token):
    """
    Exchange old_token for a new one.

    SAFE: the old token is atomically invalidated (UPDATE WHERE valid=1) before
    the new token is created.  Only the request that observes rowcount=1 proceeds.
    """
    token_record = get_valid_token(old_token)
    if not token_record:
        return {"error": "Invalid or expired token"}, 401

    user_id = token_record["user_id"]
    conn = get_db()

# vuln-code-snippet start chain_token_refresh_race_safe
    cursor = conn.execute(
        "UPDATE tokens SET valid = 0 WHERE token = ? AND valid = 1",
        (old_token,),
    )  # vuln-code-snippet safe-line chain_token_refresh_race_safe
    conn.commit()
    conn.close()
# vuln-code-snippet end chain_token_refresh_race_safe

    if cursor.rowcount == 0:
        return {"error": "Token already consumed"}, 409

    new_token = create_token(user_id)
    return {"status": "ok", "token": new_token}, 200
