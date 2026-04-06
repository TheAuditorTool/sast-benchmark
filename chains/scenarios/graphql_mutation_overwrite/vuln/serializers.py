"""GraphQL mutation input deserializer -- VULNERABLE variant.

Applies all fields from the mutation input object without restriction,
allowing the caller to set 'role' or 'subscription' in a single mutation.

CWE-915: Improperly Controlled Modification of Dynamically-Determined Object Attributes
Chain: updateUser mutation with role=admin -> user record overwritten including role
"""


# vuln-code-snippet start chain_graphql_overwrite_vuln
def deserialize_mutation_input(input_obj):
    """Return fields to apply from a GraphQL mutation input.

    VULNERABLE: all keys in input_obj are copied into the update dict,
    including 'role' and 'subscription'.
    """
    return {k: v for k, v in input_obj.items()}  # vuln-code-snippet vuln-line chain_graphql_overwrite_vuln
# vuln-code-snippet end chain_graphql_overwrite_vuln
