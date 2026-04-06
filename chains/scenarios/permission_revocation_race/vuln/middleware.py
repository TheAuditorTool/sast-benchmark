"""Permission check middleware -- VULNERABLE variant.

Loads all permissions for the requesting user into g at request start.
If an admin revokes a permission after the cache is populated but before the
protected endpoint checks it, the stale cached set still contains the revoked
permission and the operation is allowed.

Chain: request start -> g.permissions = {set} -> revoke -> g.permissions still has perm -> action allowed
Individual findings: stale permission cache within request (medium)
Chain finding: permission revocation is ineffective for in-flight requests (CWE-362, critical)
"""
import functools
from flask import g, request, jsonify
from permissions import has_permission


def load_permissions():
    """
    Cache all permissions for the current user in g.

    VULNERABLE: permissions are read once at request start.  A concurrent
    revocation modifies the database but the cached set in g is unchanged.
    """
    user_id = request.headers.get("X-User-Id")
    if user_id:
        try:
            g.user_id = int(user_id)
        except ValueError:
            g.user_id = None
    else:
        g.user_id = None

    # Stale: cache is populated now, not at point of use
    g.perm_cache = {}


def require_permission(permission):
    """Decorator that checks the cached permission set."""
    def decorator(f):
        @functools.wraps(f)
        def decorated(*args, **kwargs):
            user_id = getattr(g, "user_id", None)
            if user_id is None:
                return jsonify({"error": "Authentication required"}), 401

            # Uses cache populated at request start -- may be stale
            if permission not in g.perm_cache:
                g.perm_cache[permission] = has_permission(user_id, permission)

# vuln-code-snippet start chain_perm_revoke_race_vuln
            if not g.perm_cache[permission]:
                return jsonify({"error": "Permission denied"}), 403
            return f(*args, **kwargs)  # vuln-code-snippet vuln-line chain_perm_revoke_race_vuln
# vuln-code-snippet end chain_perm_revoke_race_vuln

        return decorated
    return decorator
