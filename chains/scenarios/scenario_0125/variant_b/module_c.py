_EDITABLE_FIELDS = {"name", "description", "public"}

def deserialize_project_settings(data):
    return {k: v for k, v in data.items() if k in _EDITABLE_FIELDS}
