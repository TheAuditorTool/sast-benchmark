"""JSON Merge Patch deserializer -- VULNERABLE variant.

Applies the RFC 7396 merge patch without filtering protected fields,
allowing a caller to set 'access_level' to 'public' on any resource
they own.

CWE-915: Improperly Controlled Modification of Dynamically-Determined Object Attributes
Chain: PATCH /resources/<id> with access_level=public -> resource exposed publicly
"""


# vuln-code-snippet start chain_merge_patch_vuln
def apply_merge_patch(resource, patch):
    """Apply a JSON Merge Patch to a resource dict.

    VULNERABLE: all non-null patch keys overwrite the resource, including
    'access_level' which controls visibility.
    """
    for key, value in patch.items():
        if value is None:
            resource.pop(key, None)
        else:
            resource[key] = value  # vuln-code-snippet vuln-line chain_merge_patch_vuln
    return resource
# vuln-code-snippet end chain_merge_patch_vuln
