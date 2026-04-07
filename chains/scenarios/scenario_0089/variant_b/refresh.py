from tokens import get_db, get_valid_token, create_token

def refresh_token(old_token):
    token_record = get_valid_token(old_token)
    if not token_record:
        return {"error": "Invalid or expired token"}, 401

    user_id = token_record["user_id"]
    conn = get_db()

# vuln-code-snippet start ChainScenario0089B
    cursor = conn.execute(
        "UPDATE tokens SET valid = 0 WHERE token = ? AND valid = 1",
        (old_token,),
    )  # vuln-code-snippet target-line ChainScenario0089B
    conn.commit()
    conn.close()
# vuln-code-snippet end ChainScenario0089B

    if cursor.rowcount == 0:
        return {"error": "Token already consumed"}, 409

    new_token = create_token(user_id)
    return {"status": "ok", "token": new_token}, 200
