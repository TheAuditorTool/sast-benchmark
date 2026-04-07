import functools
from flask import g, request, jsonify
from module_c import has_permission

def load_permissions():
    user_id = request.headers.get("X-User-Id")
    if user_id:
        try:
            g.user_id = int(user_id)
        except ValueError:
            g.user_id = None
    else:
        g.user_id = None

def require_permission(permission):
    def decorator(f):
        @functools.wraps(f)
        def decorated(*args, **kwargs):
            user_id = getattr(g, "user_id", None)
            if user_id is None:
                return jsonify({"error": "Authentication required"}), 401

            if not has_permission(user_id, permission):
                return jsonify({"error": "Permission denied"}), 403
            return f(*args, **kwargs)

        return decorated
    return decorator
