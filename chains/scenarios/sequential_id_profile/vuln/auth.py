"""Session-based authentication helpers.

This file is IDENTICAL between vuln/ and safe/ variants.
Provides get_current_user() which extracts the authenticated user id
from the Flask session.  Authorization ownership checks are the
responsibility of each route handler.
"""
import functools
from flask import session, jsonify


def get_current_user():
    """Return the authenticated user's id from the session, or None."""
    return session.get("user_id")


def login_required(f):
    """Decorator that rejects unauthenticated requests."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        if get_current_user() is None:
            return jsonify({"error": "Authentication required"}), 401
        return f(*args, **kwargs)
    return decorated
