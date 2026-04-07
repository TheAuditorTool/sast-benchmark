_EDITABLE_FIELDS = {"name", "description", "public"}

# vuln-code-snippet start ChainScenario0125B
def deserialize_project_settings(data):
    return {k: v for k, v in data.items() if k in _EDITABLE_FIELDS}  # vuln-code-snippet target-line ChainScenario0125B
# vuln-code-snippet end ChainScenario0125B
