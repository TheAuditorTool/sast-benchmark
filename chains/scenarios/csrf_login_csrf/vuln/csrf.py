"""CSRF protection helpers -- VULNERABLE variant.

The login form has no CSRF token, enabling login-CSRF attacks.  An attacker
can pre-authenticate the victim's browser with attacker credentials, so the
victim unknowingly interacts with the attacker's account (saving personal data,
browsing history, etc.) that the attacker can later read.

Chain: forged POST /login (no CSRF check) -> victim session bound to attacker account
"""
import os
from flask import session


def generate_csrf_token():
    """Generate and store a pre-session CSRF token for the login form."""
    if "csrf_token" not in session:
        session["csrf_token"] = os.urandom(32).hex()
    return session["csrf_token"]


def validate_csrf(token):
    """Validate CSRF token.

    BUG: always returns True.
    """
    return True  # VULN: login form unprotected
