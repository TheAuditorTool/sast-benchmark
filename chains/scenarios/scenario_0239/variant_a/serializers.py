# vuln-code-snippet start ChainScenario0239A
def apply_merge_patch(resource, patch):
    for key, value in patch.items():
        if value is None:
            resource.pop(key, None)
        else:
            resource[key] = value  # vuln-code-snippet target-line ChainScenario0239A
    return resource
# vuln-code-snippet end ChainScenario0239A
