"""CSRF protection helpers -- VULNERABLE variant.

validate_csrf() is a stub, allowing a forged POST to grant arbitrary permissions
to an attacker-controlled principal without the resource owner's knowledge.

Chain: forged POST -> /resources/<id>/permissions (no CSRF check) -> access granted
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
    return True  # VULN: stub
