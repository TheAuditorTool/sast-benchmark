from module_c import get_valid_token, create_token, invalidate_token

def refresh_token(old_token):
    token_record = get_valid_token(old_token)

    if not token_record:
        return {"error": "Invalid or expired token"}, 401

    user_id = token_record["user_id"]
    new_token = create_token(user_id)

    invalidate_token(old_token)

    return {"status": "ok", "token": new_token}, 200
