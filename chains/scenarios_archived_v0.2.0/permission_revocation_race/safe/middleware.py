"""Permission check middleware -- SAFE variant.

Checks permissions fresh from the database at the point of each protected operation.
There is no per-request cache, so a concurrent revocation is immediately visible
to any check that runs after it.

Chain: request -> protected op -> fresh has_permission() DB call -> correct result
Individual findings: none -- permissions checked at point of use
Chain finding: none -- revocation is immediately effective (CWE-362 mitigated)
"""
import functools
from flask import g, request, jsonify
from permissions import has_permission


def load_permissions():
    """Store the user_id in g; no permission caching."""
    user_id = request.headers.get("X-User-Id")
    if user_id:
        try:
            g.user_id = int(user_id)
        except ValueError:
            g.user_id = None
    else:
        g.user_id = None


def require_permission(permission):
    """Decorator that checks permission fresh from the database on each call."""
    def decorator(f):
        @functools.wraps(f)
        def decorated(*args, **kwargs):
            user_id = getattr(g, "user_id", None)
            if user_id is None:
                return jsonify({"error": "Authentication required"}), 401

# vuln-code-snippet start chain_perm_revoke_race_safe
            if not has_permission(user_id, permission):
                return jsonify({"error": "Permission denied"}), 403
            return f(*args, **kwargs)  # vuln-code-snippet safe-line chain_perm_revoke_race_safe
# vuln-code-snippet end chain_perm_revoke_race_safe

        return decorated
    return decorator
