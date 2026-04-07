"""Invite deserializer -- VULNERABLE variant.

Passes the full invite payload through without constraining the 'role'
field, so any caller can invite themselves as 'owner' or 'admin'.

CWE-915: Improperly Controlled Modification of Dynamically-Determined Object Attributes
Chain: POST /orgs/<id>/invite with role=owner -> membership stored with owner role
"""


# vuln-code-snippet start chain_org_role_override_vuln
def deserialize_invite(data):
    """Build a membership dict from the caller-supplied invite payload.

    VULNERABLE: 'role' is taken verbatim from data; no validation against
    permitted roles is performed.
    """
    return {  # vuln-code-snippet vuln-line chain_org_role_override_vuln
        "user_id": data.get("user_id", ""),
        "role": data.get("role", "member"),
    }
# vuln-code-snippet end chain_org_role_override_vuln
