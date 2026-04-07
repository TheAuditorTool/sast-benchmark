"""Input sanitization helpers -- SAFE variant.

clean_registration() now ignores the user-supplied role field entirely
and always assigns the default "member" role, preventing self-assignment
of elevated roles.

Chain: attacker supplies role=billing_admin -> overridden to member -> chain broken
"""
from models import VALID_ROLES


def clean_registration(data):
    """Return a sanitized registration dict.

    SAFE: role field is always set to the default 'member' regardless of input.
    """
    return {
        "username": str(data.get("username", "")).strip(),
        "email": str(data.get("email", "")).strip(),
        "org": str(data.get("org", "")).strip(),
        "role": "member",
    }
