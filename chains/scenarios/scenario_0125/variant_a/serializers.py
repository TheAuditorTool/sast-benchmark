# vuln-code-snippet start ChainScenario0125A
def deserialize_project_settings(data):
    return {k: v for k, v in data.items()}  # vuln-code-snippet target-line ChainScenario0125A
# vuln-code-snippet end ChainScenario0125A
