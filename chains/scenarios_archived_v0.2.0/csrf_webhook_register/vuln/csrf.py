"""CSRF protection helpers -- VULNERABLE variant.

validate_csrf() is a stub, so a forged POST can register an attacker-controlled
webhook URL under the victim's account to receive all future event payloads.

Chain: forged POST -> /webhooks (no CSRF check) -> attacker URL registered
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
    return True  # VULN: check is omitted
