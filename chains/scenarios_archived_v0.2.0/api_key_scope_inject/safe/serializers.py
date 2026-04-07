"""API key creation deserializer -- SAFE variant.

Restricts requested scopes to those a regular user may hold.
Privileged scopes such as 'admin' are silently removed.

CWE-915: Fixed by filtering requested scopes against a permitted set.
Chain: POST /api-keys with scopes=["admin"] -> admin scope removed -> key has read scope only
"""

_USER_SCOPES = {"read", "write"}


# vuln-code-snippet start chain_api_key_scope_safe
def deserialize_key_create(data):
    """Build an API key spec, restricting scopes to user-level values.

    SAFE: requested scopes are intersected with _USER_SCOPES; privileged
    scopes are never granted through this path.
    """
    raw_scopes = data.get("scopes", ["read"])
    allowed = [s for s in raw_scopes if s in _USER_SCOPES] or ["read"]  # vuln-code-snippet safe-line chain_api_key_scope_safe
    return {
        "name": data.get("name", "unnamed"),
        "scopes": allowed,
    }
# vuln-code-snippet end chain_api_key_scope_safe
