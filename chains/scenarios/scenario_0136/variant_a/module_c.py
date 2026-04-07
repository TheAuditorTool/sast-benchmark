_EDITABLE_FIELDS = {"payment_method_id"}

def deserialize_plan_update(data):
    return {k: v for k, v in data.items() if k in _EDITABLE_FIELDS}
