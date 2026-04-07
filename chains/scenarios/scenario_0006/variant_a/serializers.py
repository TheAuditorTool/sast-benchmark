# vuln-code-snippet start ChainScenario0006A
def deserialize_account_update(data):
    return {k: v for k, v in data.items()}  # vuln-code-snippet target-line ChainScenario0006A
# vuln-code-snippet end ChainScenario0006A
