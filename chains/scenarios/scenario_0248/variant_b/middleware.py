import functools
from flask import g, request, jsonify
from permissions import has_permission

def load_permissions():
    user_id = request.headers.get("X-User-Id")
    if user_id:
        try:
            g.user_id = int(user_id)
        except ValueError:
            g.user_id = None
    else:
        g.user_id = None

    g.perm_cache = {}

def require_permission(permission):
    def decorator(f):
        @functools.wraps(f)
        def decorated(*args, **kwargs):
            user_id = getattr(g, "user_id", None)
            if user_id is None:
                return jsonify({"error": "Authentication required"}), 401

            if permission not in g.perm_cache:
                g.perm_cache[permission] = has_permission(user_id, permission)

# vuln-code-snippet start ChainScenario0248B
            if not g.perm_cache[permission]:
                return jsonify({"error": "Permission denied"}), 403
            return f(*args, **kwargs)  # vuln-code-snippet target-line ChainScenario0248B
# vuln-code-snippet end ChainScenario0248B

        return decorated
    return decorator
