_WRITABLE_FIELDS = {"name", "content"}

# vuln-code-snippet start ChainScenario0239B
def apply_merge_patch(resource, patch):
    safe_patch = {k: v for k, v in patch.items() if k in _WRITABLE_FIELDS}  # vuln-code-snippet target-line ChainScenario0239B
    for key, value in safe_patch.items():
        if value is None:
            resource.pop(key, None)
        else:
            resource[key] = value
    return resource
# vuln-code-snippet end ChainScenario0239B
