"""Share-token authentication helpers.

This file is IDENTICAL between vuln/ and safe/ variants.
The share token is passed as a query parameter; this module validates
its existence but does NOT check access_level -- that is the
responsibility of each route handler.
"""
import functools
from flask import request, jsonify, g
from models import get_share_by_token


def require_share_token(f):
    """Decorator that validates the share token and stores the share in g."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        token = request.args.get("token") or request.headers.get("X-Share-Token")
        if not token:
            return jsonify({"error": "Share token required"}), 401
        share = get_share_by_token(token)
        if share is None:
            return jsonify({"error": "Invalid or expired token"}), 401
        g.share = share
        return f(*args, **kwargs)
    return decorated
