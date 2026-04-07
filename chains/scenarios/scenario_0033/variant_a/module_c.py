_EDITABLE_FIELDS = {"theme", "notifications_enabled"}

def deserialize_preferences(data):
    return {k: v for k, v in data.items() if k in _EDITABLE_FIELDS}
