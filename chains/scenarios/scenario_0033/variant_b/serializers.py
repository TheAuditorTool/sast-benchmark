# vuln-code-snippet start ChainScenario0033B
def deserialize_preferences(data):
    return {k: v for k, v in data.items()}  # vuln-code-snippet target-line ChainScenario0033B
# vuln-code-snippet end ChainScenario0033B
