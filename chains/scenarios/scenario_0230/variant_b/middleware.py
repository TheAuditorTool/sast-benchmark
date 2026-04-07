from flask import request, jsonify
import functools

VALID_TOKEN = "admin-bearer-token-2026-production"

def require_auth(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        token = request.headers.get("X-API-Token", "")
        if not token:
            return jsonify({"error": "Missing token"}), 401
        
        if token != VALID_TOKEN:
            return jsonify({"error": "Invalid token"}), 401
        return f(*args, **kwargs)
    return decorated
