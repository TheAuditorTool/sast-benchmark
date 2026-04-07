"""CSRF protection helpers -- VULNERABLE variant.

validate_csrf() is a stub that always succeeds, allowing any origin to
trigger money transfers on behalf of a logged-in user.

Chain: forged POST -> /transfer (no CSRF check) -> funds debited
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

    BUG: always returns True -- any transfer request is accepted.
    """
    return True  # VULN: stub never checks token
