"""Session authentication helpers for the webhook delivery service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
import functools
from flask import session, jsonify


def get_current_user_id():
    """Return the authenticated user's id from the session, or None."""
    return session.get("user_id")


def login_required(f):
    """Decorator that rejects unauthenticated requests."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        if get_current_user_id() is None:
            return jsonify({"error": "Authentication required"}), 401
        return f(*args, **kwargs)
    return decorated
