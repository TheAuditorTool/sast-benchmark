_ASSIGNABLE_ROLES = {"member", "contributor", "moderator"}

# vuln-code-snippet start ChainScenario0061B
def deserialize_invite(data):
    raw_role = data.get("role", "member")
    role = raw_role if raw_role in _ASSIGNABLE_ROLES else "member"  # vuln-code-snippet target-line ChainScenario0061B
    return {
        "user_id": data.get("user_id", ""),
        "role": role,
    }
# vuln-code-snippet end ChainScenario0061B
