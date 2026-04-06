"""JSON Merge Patch deserializer -- SAFE variant.

Applies the merge patch but first strips any keys that are not in the
set of user-writable fields, preventing 'access_level' from being
overwritten through this endpoint.

CWE-915: Fixed by stripping protected keys from the patch before applying.
Chain: PATCH /resources/<id> with access_level=public -> key stripped -> access_level unchanged
"""

_WRITABLE_FIELDS = {"name", "content"}


# vuln-code-snippet start chain_merge_patch_safe
def apply_merge_patch(resource, patch):
    """Apply a filtered JSON Merge Patch to a resource dict.

    SAFE: keys outside _WRITABLE_FIELDS are removed from the patch before
    it is applied; 'access_level' cannot be changed through this path.
    """
    safe_patch = {k: v for k, v in patch.items() if k in _WRITABLE_FIELDS}  # vuln-code-snippet safe-line chain_merge_patch_safe
    for key, value in safe_patch.items():
        if value is None:
            resource.pop(key, None)
        else:
            resource[key] = value
    return resource
# vuln-code-snippet end chain_merge_patch_safe
