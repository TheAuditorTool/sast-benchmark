"""Signup invite validation -- VULNERABLE variant.

The invite token check is present but the signup route in views.py does
not call validate_invite() before creating the account. A guest can POST
directly to /signup without a valid invite token and gain user access.

Chain: unauthenticated guest -> invite check skipped -> user account created
Individual findings: missing authentication for critical function (CWE-287)
Chain finding: guest-to-user privilege escalation
"""
from models import INVITE_TOKENS


def validate_invite(token):
    """Check that a token is in the valid invite set. Returns True if valid."""
    return token in INVITE_TOKENS


def consume_invite(token):
    """Remove token from the set after use to prevent replay."""
    INVITE_TOKENS.discard(token)
