"""Project settings deserializer -- VULNERABLE variant.

Passes all project settings fields through including 'owner_id', allowing
a member to transfer ownership to themselves.

CWE-915: Improperly Controlled Modification of Dynamically-Determined Object Attributes
Chain: PATCH /projects/<id>/settings with owner_id -> project ownership transferred
"""


# vuln-code-snippet start chain_project_owner_vuln
def deserialize_project_settings(data):
    """Build a settings update dict from caller-supplied data.

    VULNERABLE: 'owner_id' is not filtered out; any project member can
    overwrite the owner reference.
    """
    return {k: v for k, v in data.items()}  # vuln-code-snippet vuln-line chain_project_owner_vuln
# vuln-code-snippet end chain_project_owner_vuln
