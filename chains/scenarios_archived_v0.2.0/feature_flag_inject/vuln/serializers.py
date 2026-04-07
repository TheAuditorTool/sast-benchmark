"""Preferences deserializer -- VULNERABLE variant.

Copies the full request body including 'feature_flags', which may contain
admin-only flag names that unlock privileged functionality.

CWE-915: Improperly Controlled Modification of Dynamically-Determined Object Attributes
Chain: PUT /preferences with feature_flags=["beta_admin_panel"] -> admin flag enabled for user
"""


# vuln-code-snippet start chain_feature_flag_vuln
def deserialize_preferences(data):
    """Build a preferences update dict from caller-supplied data.

    VULNERABLE: the 'feature_flags' list is accepted verbatim; an
    attacker can enable admin-only flags.
    """
    return {k: v for k, v in data.items()}  # vuln-code-snippet vuln-line chain_feature_flag_vuln
# vuln-code-snippet end chain_feature_flag_vuln
