"""Input sanitization helpers -- VULNERABLE variant.

clean_registration() strips whitespace from user-supplied fields but
does not strip or override the 'role' field. The registration route
accepts whatever role value the requester supplies, enabling an attacker
to self-assign billing_admin or owner on registration.

Chain: attacker supplies role=billing_admin -> accepted verbatim -> elevated account
Individual findings: mass parameter assignment (CWE-915)
Chain finding: role parameter tampering leads to privilege escalation
"""
from models import VALID_ROLES


def clean_registration(data):
    """Return a sanitized registration dict.

    VULNERABLE: role field is passed through from user input unmodified.
    """
    return {
        "username": str(data.get("username", "")).strip(),
        "email": str(data.get("email", "")).strip(),
        "org": str(data.get("org", "")).strip(),
        "role": str(data.get("role", "member")).strip(),
    }
