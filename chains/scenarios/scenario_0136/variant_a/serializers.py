_EDITABLE_FIELDS = {"payment_method_id"}

# vuln-code-snippet start ChainScenario0136A
def deserialize_plan_update(data):
    return {k: v for k, v in data.items() if k in _EDITABLE_FIELDS}  # vuln-code-snippet target-line ChainScenario0136A
# vuln-code-snippet end ChainScenario0136A
