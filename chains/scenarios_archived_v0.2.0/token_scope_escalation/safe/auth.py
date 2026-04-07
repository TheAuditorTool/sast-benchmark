"""Token scope verification -- VULNERABLE variant.

resolve_token() looks up the API token and returns the token record.
The scope check is only performed in JavaScript/client-side tooling,
not enforced server-side. The server trusts that the client will not
call write endpoints with a read-only token.

Chain: read-scoped token -> server never checks scope -> write action executed
Individual findings: improper privilege management (CWE-269)
Chain finding: read-scope token escalates to write
"""
import functools
from flask import request, jsonify
from models import API_TOKENS


def resolve_token():
    """Return the token record for the Bearer token in Authorization header."""
    auth = request.headers.get("Authorization", "")
    if not auth.startswith("Bearer "):
        return None
    token_str = auth[7:].strip()
    return API_TOKENS.get(token_str)


def require_token(f):
    """Verify a valid API token is present; does NOT check scope."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        token = resolve_token()
        if token is None:
            return jsonify({"error": "Invalid or missing API token"}), 401
        request.token = token
        return f(*args, **kwargs)
    return decorated


def require_scope(scope):
    """Return a decorator that checks for a specific scope.

    VULNERABLE: this function exists but is never used in routes.py.
    The server relies on clients self-enforcing scope restrictions.
    """
    def decorator(f):
        @functools.wraps(f)
        def decorated(*args, **kwargs):
            token = resolve_token()
            if token is None:
                return jsonify({"error": "Invalid or missing API token"}), 401
            if scope not in token.get("scopes", []):
                return jsonify({"error": f"Scope '{scope}' required"}), 403
            request.token = token
            return f(*args, **kwargs)
        return decorated
    return decorator
