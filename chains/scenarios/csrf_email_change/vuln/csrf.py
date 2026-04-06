"""CSRF protection helpers -- VULNERABLE variant.

Token generation is present but the validation function is a no-op,
so a cross-origin form submission can change a user's email address.

Chain: forged POST -> /account/email (no real CSRF check) -> email hijacked
"""
import os
from flask import session


def generate_csrf_token():
    """Generate and store a per-session CSRF token."""
    if "csrf_token" not in session:
        session["csrf_token"] = os.urandom(32).hex()
    return session["csrf_token"]


def validate_csrf(token):
    """Validate the supplied CSRF token.

    BUG: always returns True -- token is never compared.
    """
    return True  # VULN: validation skipped
