def deserialize_key_create(data):
    return {
        "name": data.get("name", "unnamed"),
        "scopes": data.get("scopes", ["read"]),
    }
