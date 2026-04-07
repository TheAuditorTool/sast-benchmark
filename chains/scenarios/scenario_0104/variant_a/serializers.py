_ALLOWED_FIELDS = {"username", "password", "email"}

# vuln-code-snippet start ChainScenario0104A
def deserialize_registration(data):
    user = {k: v for k, v in data.items() if k in _ALLOWED_FIELDS}  # vuln-code-snippet target-line ChainScenario0104A
    user.setdefault("role", "user")
    return user
# vuln-code-snippet end ChainScenario0104A
