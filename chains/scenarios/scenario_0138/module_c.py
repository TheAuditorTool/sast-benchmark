_EDITABLE_TOP = {"username"}
_EDITABLE_SETTINGS = {"theme"}

def deserialize_profile_update(data, existing_settings):
    top_level = {k: v for k, v in data.items() if k in _EDITABLE_TOP}
    if "settings" in data and isinstance(data["settings"], dict):
        safe_settings = {k: v for k, v in data["settings"].items() if k in _EDITABLE_SETTINGS}
        existing_settings.update(safe_settings)
        top_level["settings"] = existing_settings
    return top_level
