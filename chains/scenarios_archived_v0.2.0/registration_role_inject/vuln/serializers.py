"""Registration input deserializer -- VULNERABLE variant.

Accepts all fields from the incoming JSON body without filtering,
allowing a caller to inject arbitrary keys such as 'role'.

CWE-915: Improperly Controlled Modification of Dynamically-Determined Object Attributes
Chain: POST /register with role field -> user created with attacker-chosen role
"""


# vuln-code-snippet start chain_registration_role_vuln
def deserialize_registration(data):
    """Return a user dict built directly from caller-supplied data.

    VULNERABLE: no allowlist applied; every key in data is copied into
    the returned dict, including privileged fields like 'role'.
    """
    return {k: v for k, v in data.items()}  # vuln-code-snippet vuln-line chain_registration_role_vuln
# vuln-code-snippet end chain_registration_role_vuln
