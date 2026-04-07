def apply_merge_patch(resource, patch):
    for key, value in patch.items():
        if value is None:
            resource.pop(key, None)
        else:
            resource[key] = value
    return resource
