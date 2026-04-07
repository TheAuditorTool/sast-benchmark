_EDITABLE_FIELDS = {"display_name"}

def deserialize_account_update(data):
    return {k: v for k, v in data.items() if k in _EDITABLE_FIELDS}
