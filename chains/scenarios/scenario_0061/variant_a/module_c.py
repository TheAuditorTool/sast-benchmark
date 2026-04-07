def deserialize_invite(data):
    return {
        "user_id": data.get("user_id", ""),
        "role": data.get("role", "member"),
    }
