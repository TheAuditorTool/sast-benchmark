_MUTABLE_FIELDS = {"email", "display_name"}

# vuln-code-snippet start ChainScenario0240A
def deserialize_mutation_input(input_obj):
    return {k: v for k, v in input_obj.items() if k in _MUTABLE_FIELDS}  # vuln-code-snippet target-line ChainScenario0240A
# vuln-code-snippet end ChainScenario0240A
