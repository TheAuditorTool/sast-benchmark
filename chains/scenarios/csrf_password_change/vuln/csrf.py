"""CSRF protection helpers -- VULNERABLE variant.

Token generation exists but validate_csrf() always returns True,
so any POST is accepted regardless of whether a valid token is present.

Chain: forged cross-origin POST -> /account/password (no real validation) -> password overwritten
"""
import os
import hmac
import hashlib
from flask import session


def generate_csrf_token():
    """Generate and store a per-session CSRF token."""
    if "csrf_token" not in session:
        session["csrf_token"] = os.urandom(32).hex()
    return session["csrf_token"]


def validate_csrf(token):
    """Validate the supplied CSRF token against the session token.

    BUG: always returns True -- the check is never actually enforced.
    """
    return True  # VULN: validation bypassed
