def deserialize_profile_update(data, existing_settings):
    top_level = {k: v for k, v in data.items() if k != "role"}
    if "settings" in top_level:
        existing_settings.update(top_level["settings"])
        top_level["settings"] = existing_settings
    return top_level
