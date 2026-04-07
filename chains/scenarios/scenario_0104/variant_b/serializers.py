# vuln-code-snippet start ChainScenario0104B
def deserialize_registration(data):
    return {k: v for k, v in data.items()}  # vuln-code-snippet target-line ChainScenario0104B
# vuln-code-snippet end ChainScenario0104B
