"""Preferences deserializer -- SAFE variant.

Allowlists only user-configurable preference fields.  The 'feature_flags'
key is excluded; flag assignment is performed by server-side subscription
or admin tooling only.

CWE-915: Fixed by excluding 'feature_flags' from the writable field set.
Chain: PUT /preferences with feature_flags -> feature_flags ignored -> no admin flags set
"""

_EDITABLE_FIELDS = {"theme", "notifications_enabled"}


# vuln-code-snippet start chain_feature_flag_safe
def deserialize_preferences(data):
    """Build a preferences update dict, excluding privileged fields.

    SAFE: 'feature_flags' is not in _EDITABLE_FIELDS and is dropped.
    """
    return {k: v for k, v in data.items() if k in _EDITABLE_FIELDS}  # vuln-code-snippet safe-line chain_feature_flag_safe
# vuln-code-snippet end chain_feature_flag_safe
