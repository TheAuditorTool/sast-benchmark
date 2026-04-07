_EDITABLE_FIELDS = {"email", "bio", "display_name"}

def validate_profile_update(data):
    return {k: v for k, v in data.items() if k in _EDITABLE_FIELDS}
