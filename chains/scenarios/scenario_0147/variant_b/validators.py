_EDITABLE_FIELDS = {"email", "bio", "display_name"}

# vuln-code-snippet start ChainScenario0147B
def validate_profile_update(data):
    return {k: v for k, v in data.items() if k in _EDITABLE_FIELDS}  # vuln-code-snippet target-line ChainScenario0147B
# vuln-code-snippet end ChainScenario0147B
