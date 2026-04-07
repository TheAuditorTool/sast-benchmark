# vuln-code-snippet start ChainScenario0126A
def deserialize_key_create(data):
    return {  # vuln-code-snippet target-line ChainScenario0126A
        "name": data.get("name", "unnamed"),
        "scopes": data.get("scopes", ["read"]),
    }
# vuln-code-snippet end ChainScenario0126A
