import functools
from flask import request, jsonify
from module_b import API_TOKENS

def resolve_token():
    auth = request.headers.get("Authorization", "")
    if not auth.startswith("Bearer "):
        return None
    token_str = auth[7:].strip()
    return API_TOKENS.get(token_str)

def require_token(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        token = resolve_token()
        if token is None:
            return jsonify({"error": "Invalid or missing API token"}), 401
        request.token = token
        return f(*args, **kwargs)
    return decorated

def require_scope(scope):
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
