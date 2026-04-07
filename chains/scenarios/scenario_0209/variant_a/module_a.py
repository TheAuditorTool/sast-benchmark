import functools
from flask import request, jsonify
from module_b import PROFILES

def get_current_user_id():
    return request.headers.get("X-User-Id", "")

def require_own_profile(f):
    @functools.wraps(f)
    def decorated(user_id, *args, **kwargs):
        caller_id = get_current_user_id()
        if not caller_id or caller_id not in PROFILES:
            return jsonify({"error": "Authentication required"}), 401
        if caller_id != user_id:
            return jsonify({"error": "Cannot edit another user's profile"}), 403
        request.current_user_id = caller_id
        return f(user_id, *args, **kwargs)
    return decorated
