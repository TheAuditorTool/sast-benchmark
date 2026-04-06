"""Nested profile settings deserializer -- SAFE variant.

Performs a deep merge but allowlists the keys that may be written inside
the nested 'settings' object, preventing the role from being overwritten
through the nested path.

CWE-915: Fixed by allowlisting keys within the nested settings merge.
Chain: PATCH /profile with settings.role=admin -> role key ignored in nested merge
"""

_EDITABLE_TOP = {"username"}
_EDITABLE_SETTINGS = {"theme"}


# vuln-code-snippet start chain_nested_role_safe
def deserialize_profile_update(data, existing_settings):
    """Merge caller-supplied data, allowlisting nested settings keys.

    SAFE: only _EDITABLE_SETTINGS keys are merged from the nested object,
    so 'role' cannot be overwritten regardless of nesting depth.
    """
    top_level = {k: v for k, v in data.items() if k in _EDITABLE_TOP}
    if "settings" in data and isinstance(data["settings"], dict):
        safe_settings = {k: v for k, v in data["settings"].items() if k in _EDITABLE_SETTINGS}  # vuln-code-snippet safe-line chain_nested_role_safe
        existing_settings.update(safe_settings)
        top_level["settings"] = existing_settings
    return top_level
# vuln-code-snippet end chain_nested_role_safe
