_EDITABLE_FIELDS = {"display_name"}

# vuln-code-snippet start ChainScenario0006B
def deserialize_account_update(data):
    return {k: v for k, v in data.items() if k in _EDITABLE_FIELDS}  # vuln-code-snippet target-line ChainScenario0006B
# vuln-code-snippet end ChainScenario0006B
