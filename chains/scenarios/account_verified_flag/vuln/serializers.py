"""Account update deserializer -- VULNERABLE variant.

Copies all fields from the request body including 'email_verified',
which should only be set by the verification email flow.

CWE-915: Improperly Controlled Modification of Dynamically-Determined Object Attributes
Chain: PATCH /account with email_verified=true -> account bypasses verification gate
"""


# vuln-code-snippet start chain_verified_flag_vuln
def deserialize_account_update(data):
    """Build an update dict from caller-supplied data.

    VULNERABLE: 'email_verified' passes through unchanged, letting the
    caller self-certify their own email address.
    """
    return {k: v for k, v in data.items()}  # vuln-code-snippet vuln-line chain_verified_flag_vuln
# vuln-code-snippet end chain_verified_flag_vuln
