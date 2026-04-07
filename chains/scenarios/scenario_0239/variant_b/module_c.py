_WRITABLE_FIELDS = {"name", "content"}

def apply_merge_patch(resource, patch):
    safe_patch = {k: v for k, v in patch.items() if k in _WRITABLE_FIELDS}
    for key, value in safe_patch.items():
        if value is None:
            resource.pop(key, None)
        else:
            resource[key] = value
    return resource
