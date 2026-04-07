# vuln-code-snippet start ChainScenario0154B
def deserialize_profile_update(data, existing_settings):
    top_level = {k: v for k, v in data.items() if k != "role"}
    if "settings" in top_level:
        existing_settings.update(top_level["settings"])  # vuln-code-snippet target-line ChainScenario0154B
        top_level["settings"] = existing_settings
    return top_level
# vuln-code-snippet end ChainScenario0154B
