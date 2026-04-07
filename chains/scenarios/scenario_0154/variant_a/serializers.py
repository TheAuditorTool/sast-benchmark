_EDITABLE_TOP = {"username"}
_EDITABLE_SETTINGS = {"theme"}

# vuln-code-snippet start ChainScenario0154A
def deserialize_profile_update(data, existing_settings):
    top_level = {k: v for k, v in data.items() if k in _EDITABLE_TOP}
    if "settings" in data and isinstance(data["settings"], dict):
        safe_settings = {k: v for k, v in data["settings"].items() if k in _EDITABLE_SETTINGS}  # vuln-code-snippet target-line ChainScenario0154A
        existing_settings.update(safe_settings)
        top_level["settings"] = existing_settings
    return top_level
# vuln-code-snippet end ChainScenario0154A
