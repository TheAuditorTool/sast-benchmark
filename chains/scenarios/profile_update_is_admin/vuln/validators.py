"""Profile update validator -- VULNERABLE variant.

Performs no field filtering on the incoming PATCH body, so callers can
include 'is_admin': true alongside legitimate profile fields.

CWE-915: Improperly Controlled Modification of Dynamically-Determined Object Attributes
Chain: PATCH /profile with is_admin -> user gains admin flag
"""


# vuln-code-snippet start chain_profile_is_admin_vuln
def validate_profile_update(data):
    """Return validated fields to apply.

    VULNERABLE: every key supplied by the caller passes through,
    including 'is_admin' and other privileged attributes.
    """
    return {k: v for k, v in data.items()}  # vuln-code-snippet vuln-line chain_profile_is_admin_vuln
# vuln-code-snippet end chain_profile_is_admin_vuln
