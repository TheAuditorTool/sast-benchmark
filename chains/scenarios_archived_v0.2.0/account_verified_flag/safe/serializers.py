"""Account update deserializer -- SAFE variant.

Allowlists only display name and other user-editable fields.
'email_verified' is excluded and can only be set by the verification flow.

CWE-915: Fixed by excluding 'email_verified' from the writable field set.
Chain: PATCH /account with email_verified=true -> field ignored -> status unchanged
"""

_EDITABLE_FIELDS = {"display_name"}


# vuln-code-snippet start chain_verified_flag_safe
def deserialize_account_update(data):
    """Build an account update dict, excluding flag fields.

    SAFE: 'email_verified' is not in _EDITABLE_FIELDS and is dropped.
    """
    return {k: v for k, v in data.items() if k in _EDITABLE_FIELDS}  # vuln-code-snippet safe-line chain_verified_flag_safe
# vuln-code-snippet end chain_verified_flag_safe
