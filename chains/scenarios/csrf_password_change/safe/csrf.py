"""CSRF protection helpers -- SAFE variant.

validate_csrf() performs a constant-time comparison of the submitted token
against the session-stored token, breaking the CSRF chain.

Chain: forged cross-origin POST -> /account/password -> 403 (token mismatch)
"""
import os
import hmac
from flask import session


def generate_csrf_token():
    """Generate and store a per-session CSRF token."""
    if "csrf_token" not in session:
        session["csrf_token"] = os.urandom(32).hex()
    return session["csrf_token"]


def validate_csrf(token):
    """Validate the supplied CSRF token against the session token.

    Uses hmac.compare_digest to prevent timing attacks.
    """
    expected = session.get("csrf_token", "")
    if not token or not expected:
        return False
    return hmac.compare_digest(expected, token)
