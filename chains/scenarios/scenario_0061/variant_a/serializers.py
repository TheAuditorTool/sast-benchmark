# vuln-code-snippet start ChainScenario0061A
def deserialize_invite(data):
    return {  # vuln-code-snippet target-line ChainScenario0061A
        "user_id": data.get("user_id", ""),
        "role": data.get("role", "member"),
    }
# vuln-code-snippet end ChainScenario0061A
