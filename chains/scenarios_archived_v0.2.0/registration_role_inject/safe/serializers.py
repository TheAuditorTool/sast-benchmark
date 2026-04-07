"""Registration input deserializer -- SAFE variant.

Applies an explicit allowlist of permitted fields.  Privileged fields
such as 'role' are never copied from caller-supplied data; new accounts
always receive the default 'user' role.

CWE-915: Fixed by allowlisting writable fields.
Chain: POST /register with role field -> role field ignored -> user created as 'user'
"""

_ALLOWED_FIELDS = {"username", "password", "email"}


# vuln-code-snippet start chain_registration_role_safe
def deserialize_registration(data):
    """Return a user dict limited to allowed fields, with a safe default role.

    SAFE: only keys in _ALLOWED_FIELDS are transferred; 'role' is always 'user'.
    """
    user = {k: v for k, v in data.items() if k in _ALLOWED_FIELDS}  # vuln-code-snippet safe-line chain_registration_role_safe
    user.setdefault("role", "user")
    return user
# vuln-code-snippet end chain_registration_role_safe
