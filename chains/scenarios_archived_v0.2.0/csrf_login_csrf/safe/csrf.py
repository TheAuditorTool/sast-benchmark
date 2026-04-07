"""CSRF protection helpers -- SAFE variant.

validate_csrf() does a real constant-time comparison, blocking forged login requests.

Chain: forged POST /login -> 403 (token mismatch) -> blocked
"""
import os
import hmac
from flask import session


def generate_csrf_token():
    """Generate and store a pre-session CSRF token for the login form."""
    if "csrf_token" not in session:
        session["csrf_token"] = os.urandom(32).hex()
    return session["csrf_token"]


def validate_csrf(token):
    """Validate the supplied CSRF token against the session token."""
    expected = session.get("csrf_token", "")
    if not token or not expected:
        return False
    return hmac.compare_digest(expected, token)
