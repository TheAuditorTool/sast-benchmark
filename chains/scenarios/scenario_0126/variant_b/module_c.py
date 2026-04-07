_USER_SCOPES = {"read", "write"}

def deserialize_key_create(data):
    raw_scopes = data.get("scopes", ["read"])
    allowed = [s for s in raw_scopes if s in _USER_SCOPES] or ["read"]
    return {
        "name": data.get("name", "unnamed"),
        "scopes": allowed,
    }
