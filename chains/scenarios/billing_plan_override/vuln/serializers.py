"""Billing plan update deserializer -- VULNERABLE variant.

Copies all fields from the request body, allowing a caller to set 'tier'
to 'enterprise' without going through a payment flow.

CWE-915: Improperly Controlled Modification of Dynamically-Determined Object Attributes
Chain: PUT /accounts/<id>/plan with tier=enterprise -> account upgraded for free
"""


# vuln-code-snippet start chain_billing_plan_vuln
def deserialize_plan_update(data):
    """Build an account update dict from caller-supplied data.

    VULNERABLE: 'tier' is accepted directly from the request body with
    no verification that a corresponding payment has been made.
    """
    return {k: v for k, v in data.items()}  # vuln-code-snippet vuln-line chain_billing_plan_vuln
# vuln-code-snippet end chain_billing_plan_vuln
