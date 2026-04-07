"""Billing plan update deserializer -- SAFE variant.

Allowlists only the payment_method_id field; the 'tier' field is never
accepted from the request body and must be set by server-side payment
confirmation logic.

CWE-915: Fixed by removing 'tier' from the writable field set.
Chain: PUT /accounts/<id>/plan with tier=enterprise -> tier ignored -> tier unchanged
"""

_EDITABLE_FIELDS = {"payment_method_id"}


# vuln-code-snippet start chain_billing_plan_safe
def deserialize_plan_update(data):
    """Build an account update dict, excluding privileged fields.

    SAFE: 'tier' is not in _EDITABLE_FIELDS and is therefore never applied
    from caller input.
    """
    return {k: v for k, v in data.items() if k in _EDITABLE_FIELDS}  # vuln-code-snippet safe-line chain_billing_plan_safe
# vuln-code-snippet end chain_billing_plan_safe
