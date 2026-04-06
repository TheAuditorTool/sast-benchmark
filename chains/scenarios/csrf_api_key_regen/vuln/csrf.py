"""CSRF protection helpers -- VULNERABLE variant.

validate_csrf() is a no-op, allowing any cross-origin POST to rotate
the victim's API key, giving the attacker access to the victim's API key slot.

Chain: forged POST -> /account/api-key/regenerate (no CSRF check) -> new key stored,
       attacker reads new key from their session or via secondary leak
"""
import os
from flask import session


def generate_csrf_token():
    """Generate and store a per-session CSRF token."""
    if "csrf_token" not in session:
        session["csrf_token"] = os.urandom(32).hex()
    return session["csrf_token"]


def validate_csrf(token):
    """Validate CSRF token.

    BUG: always returns True.
    """
    return True  # VULN: never checks token
