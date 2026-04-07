"""GraphQL mutation input deserializer -- SAFE variant.

Allowlists only the fields a regular user may update through the mutation.
Privileged fields such as 'role' and 'subscription' require separate flows
with additional authorisation.

CWE-915: Fixed by allowlisting mutation input fields.
Chain: updateUser mutation with role=admin -> role field stripped -> role unchanged
"""

_MUTABLE_FIELDS = {"email", "display_name"}


# vuln-code-snippet start chain_graphql_overwrite_safe
def deserialize_mutation_input(input_obj):
    """Return only the subset of fields a user may modify via mutation.

    SAFE: 'role' and 'subscription' are not in _MUTABLE_FIELDS and are dropped.
    """
    return {k: v for k, v in input_obj.items() if k in _MUTABLE_FIELDS}  # vuln-code-snippet safe-line chain_graphql_overwrite_safe
# vuln-code-snippet end chain_graphql_overwrite_safe
