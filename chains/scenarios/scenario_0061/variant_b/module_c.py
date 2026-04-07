_ASSIGNABLE_ROLES = {"member", "contributor", "moderator"}

def deserialize_invite(data):
    raw_role = data.get("role", "member")
    role = raw_role if raw_role in _ASSIGNABLE_ROLES else "member"
    return {
        "user_id": data.get("user_id", ""),
        "role": role,
    }
