"""Invite deserializer -- SAFE variant.

Constrains the 'role' field to the set of roles that an inviter may
assign.  Elevated roles such as 'owner' can only be set through a
separate ownership-transfer flow.

CWE-915: Fixed by validating role against permitted values.
Chain: POST /orgs/<id>/invite with role=owner -> role rejected or defaulted to 'member'
"""

_ASSIGNABLE_ROLES = {"member", "contributor", "moderator"}


# vuln-code-snippet start chain_org_role_override_safe
def deserialize_invite(data):
    """Build a membership dict, constraining role to assignable values.

    SAFE: role is checked against the permitted set; unknown roles fall
    back to 'member'.
    """
    raw_role = data.get("role", "member")
    role = raw_role if raw_role in _ASSIGNABLE_ROLES else "member"  # vuln-code-snippet safe-line chain_org_role_override_safe
    return {
        "user_id": data.get("user_id", ""),
        "role": role,
    }
# vuln-code-snippet end chain_org_role_override_safe
