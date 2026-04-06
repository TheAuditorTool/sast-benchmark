"""API key creation deserializer -- VULNERABLE variant.

Copies all fields from the request, including a 'scopes' list that the
caller can set to ['admin', 'billing'] to gain elevated access.

CWE-915: Improperly Controlled Modification of Dynamically-Determined Object Attributes
Chain: POST /api-keys with scopes=["admin"] -> key created with admin scope
"""


# vuln-code-snippet start chain_api_key_scope_vuln
def deserialize_key_create(data):
    """Build an API key spec from caller-supplied data.

    VULNERABLE: 'scopes' is accepted verbatim from the request body,
    allowing the caller to request privileged scopes such as 'admin'.
    """
    return {  # vuln-code-snippet vuln-line chain_api_key_scope_vuln
        "name": data.get("name", "unnamed"),
        "scopes": data.get("scopes", ["read"]),
    }
# vuln-code-snippet end chain_api_key_scope_vuln
