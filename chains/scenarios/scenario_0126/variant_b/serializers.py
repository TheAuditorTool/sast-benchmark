_USER_SCOPES = {"read", "write"}

# vuln-code-snippet start ChainScenario0126B
def deserialize_key_create(data):
    raw_scopes = data.get("scopes", ["read"])
    allowed = [s for s in raw_scopes if s in _USER_SCOPES] or ["read"]  # vuln-code-snippet target-line ChainScenario0126B
    return {
        "name": data.get("name", "unnamed"),
        "scopes": allowed,
    }
# vuln-code-snippet end ChainScenario0126B
