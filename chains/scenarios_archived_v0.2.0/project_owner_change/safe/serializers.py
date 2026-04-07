"""Project settings deserializer -- SAFE variant.

Excludes 'owner_id' from the writable field set.  Ownership transfer
requires a dedicated endpoint with additional verification.

CWE-915: Fixed by removing 'owner_id' from writable fields.
Chain: PATCH /projects/<id>/settings with owner_id -> owner_id ignored -> ownership unchanged
"""

_EDITABLE_FIELDS = {"name", "description", "public"}


# vuln-code-snippet start chain_project_owner_safe
def deserialize_project_settings(data):
    """Build a settings update dict, excluding ownership fields.

    SAFE: 'owner_id' is not in _EDITABLE_FIELDS and is therefore dropped.
    """
    return {k: v for k, v in data.items() if k in _EDITABLE_FIELDS}  # vuln-code-snippet safe-line chain_project_owner_safe
# vuln-code-snippet end chain_project_owner_safe
