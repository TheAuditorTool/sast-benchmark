"""CSRF protection helpers -- VULNERABLE variant.

The middleware is wired up but validate_csrf() performs no real check,
allowing a forged POST to create new admin accounts.

Chain: forged POST -> /admin/users (no CSRF check) -> new admin account created
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
    return True  # VULN: no real check
