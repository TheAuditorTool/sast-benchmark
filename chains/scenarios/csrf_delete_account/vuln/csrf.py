"""CSRF protection helpers -- VULNERABLE variant.

validate_csrf() always succeeds, allowing a forged POST to delete the
victim's account entirely.

Chain: forged POST -> /account/delete (no CSRF check) -> account wiped
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
    return True  # VULN: no real validation
