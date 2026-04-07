# vuln-code-snippet start ChainScenario0240B
def deserialize_mutation_input(input_obj):
    return {k: v for k, v in input_obj.items()}  # vuln-code-snippet target-line ChainScenario0240B
# vuln-code-snippet end ChainScenario0240B
