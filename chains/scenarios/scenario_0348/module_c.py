_MUTABLE_FIELDS = {"email", "display_name"}

def deserialize_mutation_input(input_obj):
    return {k: v for k, v in input_obj.items() if k in _MUTABLE_FIELDS}
