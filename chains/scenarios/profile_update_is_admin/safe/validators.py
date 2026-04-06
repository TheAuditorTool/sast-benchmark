"""Profile update validator -- SAFE variant.

Allowlists only user-editable fields; 'is_admin' and other privileged
attributes are stripped before the update is applied.

CWE-915: Fixed by allowlisting writable profile fields.
Chain: PATCH /profile with is_admin -> is_admin stripped -> privilege unchanged
"""

_EDITABLE_FIELDS = {"email", "bio", "display_name"}


# vuln-code-snippet start chain_profile_is_admin_safe
def validate_profile_update(data):
    """Return only the subset of fields a user may edit.

    SAFE: 'is_admin' and any other privileged key are silently dropped.
    """
    return {k: v for k, v in data.items() if k in _EDITABLE_FIELDS}  # vuln-code-snippet safe-line chain_profile_is_admin_safe
# vuln-code-snippet end chain_profile_is_admin_safe
