_EDITABLE_FIELDS = {"theme", "notifications_enabled"}

# vuln-code-snippet start ChainScenario0033A
def deserialize_preferences(data):
    return {k: v for k, v in data.items() if k in _EDITABLE_FIELDS}  # vuln-code-snippet target-line ChainScenario0033A
# vuln-code-snippet end ChainScenario0033A
