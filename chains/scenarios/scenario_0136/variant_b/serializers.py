# vuln-code-snippet start ChainScenario0136B
def deserialize_plan_update(data):
    return {k: v for k, v in data.items()}  # vuln-code-snippet target-line ChainScenario0136B
# vuln-code-snippet end ChainScenario0136B
