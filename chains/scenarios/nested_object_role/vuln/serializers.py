"""Nested profile settings deserializer -- VULNERABLE variant.

Performs a deep merge of user.settings with the caller-supplied body.
A naive filter strips top-level 'role', but the attacker can supply
{'settings': {'role': 'admin'}} to reach the nested field directly.

CWE-915: Improperly Controlled Modification of Dynamically-Determined Object Attributes
Chain: PATCH /profile with settings.role=admin -> nested role overwritten bypassing filter
"""


# vuln-code-snippet start chain_nested_role_vuln
def deserialize_profile_update(data, existing_settings):
    """Merge caller-supplied data into existing settings.

    VULNERABLE: top-level 'role' is stripped, but nested settings are merged
    wholesale, so {'settings': {'role': 'admin'}} bypasses the check.
    """
    top_level = {k: v for k, v in data.items() if k != "role"}
    if "settings" in top_level:
        existing_settings.update(top_level["settings"])  # vuln-code-snippet vuln-line chain_nested_role_vuln
        top_level["settings"] = existing_settings
    return top_level
# vuln-code-snippet end chain_nested_role_vuln
